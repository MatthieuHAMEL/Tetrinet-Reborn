use bevy::{input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion}, prelude::*, window::{CursorGrabMode, PrimaryWindow, WindowFocused}};

use bevy::log::LogPlugin;

// May be used later 
enum CellColor { Blue, Cyan, Green, Magenta, Purple, Red, Yellow }

#[derive(Copy, Clone)]
struct Cell {
  color: Color,
  sprite: Option<Entity> // To despawn the sprite when needed. Is it ECS-friendly? TODO
}

// Those are consts for now but it may be customizable in the future
const GRID_HEIGHT: usize = 20;
const GRID_WIDTH: usize = 10;

#[derive(Resource)]
struct Grid {
  grid: [[Option<Cell>; GRID_WIDTH]; GRID_HEIGHT],
  // Dimensions in world unit:
  block_sz: f32,
  width_sz: f32,
  height_sz: f32,
}

impl Grid {
  pub fn new() -> Self {
    const DEFAULT_BLOCK_SZ : f32 = 26.0;
    let mut grid = [[None; GRID_WIDTH]; GRID_HEIGHT];
    Self {
      grid,
      block_sz: DEFAULT_BLOCK_SZ,
      width_sz: DEFAULT_BLOCK_SZ * GRID_WIDTH as f32,
      height_sz: DEFAULT_BLOCK_SZ * GRID_HEIGHT as f32 
    }
  }
}

#[derive(Component)]
struct GridZoneMarker;

#[derive(Component)]
struct ActiveBlock; // To mark sprites that are subject to gravity and should be destroyed every second

#[derive(Component)]
struct PassiveBlock {x: usize, y: usize}

#[derive(Event)]
struct NeedTetroEvent;

/* This event is triggered when the grid (the passive blocks) changes, i.e. :
- When a tetro is locked after hitting the ground
- When a line is cleared
When it is triggered, we want to update the sprite positions or clear the sprites that disappeared.
For now it is a unit struct but it could contain values like the number of the line that disappeared
*/
#[derive(Event)]
struct GridChangedEvent;

#[derive(Resource)]
struct GravityTimer(Timer);

enum TetroKind { T, L, J, S, Z, O, I }

enum TetroRot { Base, R1, R2, R3 }

// A collection of square bricks, forming a shape like L or T
#[derive(Component)]
struct Tetro {
  coos: [(usize, usize); 4],
  kind: TetroKind,
  config: TetroRot, // Necessary to calculate the next rotation
}


// SYSTEMS //

// Simple camera
fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2d::default());
}

/*
This is the only place where I do math with the window size.
To draw blocks, I'll have a coordinate system attached to the grid
*/
fn spawn_grid_zone(
  mut commands: Commands,
  window: Single<&Window, With<PrimaryWindow>>,
  grid: Res<Grid>) {
  // Let the grid stick to the left border of the window, vertically centered
  let window_width = window.width() as f32;
  let x = -window_width / 2.0 + grid.width_sz / 2.0;
  
  commands.spawn((
    Sprite {
      color: Color::linear_rgb(0.1, 0.1, 0.1),
      custom_size: Some(Vec2::new(grid.width_sz, grid.height_sz)),
      ..Default::default()
    },
    Transform::from_xyz(x, 0.0, -1.0), // behind the blocks
    GridZoneMarker
    //Name::new("GridRoot"),
  ));
}

// Helper to get the real transform, function of the block pos and grid characteristics
fn block_transform(col: usize, row: usize, grid: &Grid) -> Transform {
  let x = col as f32 * grid.block_sz + grid.block_sz / 2.0 - grid.width_sz / 2.0;
  let y = -(row as f32 * grid.block_sz + grid.block_sz / 2.0) + grid.height_sz / 2.0;

  Transform::from_xyz(x, y, 0.0)
}

fn spawn_passive_block(mut commands: Commands, col: usize, row: usize, color: Color, grid: &Grid, grid_zone: Entity) {
  commands.spawn((
    Sprite {
      color,
      custom_size: Some(Vec2::splat(grid.block_sz)),
      ..Default::default()
    },
    block_transform(col, row, &grid),
    ChildOf(grid_zone),
  ));
}

// Draws the grid and the passive blocks
fn draw_grid(
  mut commands: Commands,
  grid_zone: Single<Entity, With<GridZoneMarker>>, // The entity returned by spawn_grid_root
  grid: Res<Grid>,
){
  for row in 0..GRID_HEIGHT {
    for col in 0..GRID_WIDTH {
      if let Some(cell) = &grid.grid[row][col] {
        // Local position relative to grid root, origin is top-left
        commands.spawn((
          Sprite {
            color: cell.color,
            custom_size: Some(Vec2::splat(grid.block_sz)),
            ..Default::default()
          },
          block_transform(col, row, &grid),
//	  ActiveBlock,
          ChildOf(*grid_zone),
        ));
      }
    }
  }
}

// On NeedTetroEvent
fn spawn_tetro(trigger: Trigger<NeedTetroEvent>, mut commands: Commands, grid: Res<Grid>) {
  // Get the middle coordinate of the grid
  let middle: usize = GRID_WIDTH / 2; // TODO wrong 
  debug!("Spawning tetro at column {}", middle);

  // For now just I spawn a T block
  // TODO create a helper function with default pos for all types of blocks
  // TODO 2 : here I could check that my new tetro's coordinates don't overlap with the grid
  commands.spawn(Tetro { coos: [(middle-1, 0), (middle, 0), (middle+1, 0), (middle, 1)], kind: TetroKind::T, config: TetroRot::Base });
}

// HELPER
fn tetro_can_move_down(tetro: &Tetro, grid: &Grid) -> bool {
  tetro.coos.iter().all(|coord| {
    if coord.1 == GRID_HEIGHT - 1 { // floor -> can't move down!
      return false;
    }
    grid.grid[coord.1 + 1][coord.0].is_none()
  })
}

// Grid is filled only when we LOCK the tetromino
fn apply_gravity(
  mut commands: Commands, mut tetros: Query<(Entity, &mut Tetro)>,
  mut grid: ResMut<Grid>,
  mut gravity_timer: ResMut<GravityTimer>, time: Res<Time>)
{
  gravity_timer.0.tick(time.delta());
  if !gravity_timer.0.finished() {
    return;
  }
  
  for (entity, mut tetro) in tetros.iter_mut() {
    if tetro_can_move_down(&tetro, &grid) {
      // move down
      for coord in tetro.coos.iter_mut() {
        coord.1 += 1;
      }
    } else {
      // lock in grid and despawn tetro
      for coord in tetro.coos.iter() {
        grid.grid[coord.1][coord.0] = Some(Cell { color: LinearRgba::BLUE.into(), sprite: None });
      }
      // Despawn the tetro
      commands.entity(entity).despawn();
      // Emit event to spawn a new tetro
      commands.trigger(NeedTetroEvent);
      // Time to update the grid (TODO)
    } // TODO -> blocks (sprites) in the grid shouldn't be ActiveBlocks anymore !
  }
}

fn clear_moving_blocks(mut commands: Commands, blocks: Query<Entity, With<ActiveBlock>>) {
  for entity in blocks.iter() {
    commands.entity(entity).despawn();
  }
}

fn draw_tetros(
  mut commands: Commands,
  grid_zone: Single<Entity, With<GridZoneMarker>>,
  tetros: Query<&Tetro>,
  grid: Res<Grid>)
{
  for tetro in tetros.iter() {
     for (col, row) in tetro.coos.iter() {
       commands.spawn((
         Sprite {
           color: LinearRgba::GREEN.into(), // TODO meaningful color
           custom_size: Some(Vec2::splat(grid.block_sz)),
           ..Default::default()
         },
         block_transform(*col, *row, &grid),
	 ActiveBlock, 
         ChildOf(*grid_zone),
       ));
     }
  }
}

fn send_initial_tetro_event(mut commands: Commands) {
  debug!("send_initial_tetro_event");
  commands.trigger(NeedTetroEvent);
}

fn main() {
  let mut app = App::new();
  app.add_plugins(DefaultPlugins.set(LogPlugin {
    filter: "warn,wgpu_core=warn,wgpu_hal=warn,tetrinet=debug".into(),
    level: bevy::log::Level::DEBUG,
    custom_layer: |_| None,
  }));

  debug!("TETRINET BEGIN");
  
  // Systems (functions) that run only at startup
  app.add_systems(
    Startup,
    (spawn_camera,
     spawn_grid_zone.after(spawn_camera),
     send_initial_tetro_event.after(spawn_grid_zone))
  );

  // Systems that run once per frame
  app.add_systems(
    Update, 
    (draw_grid , /* systems with .after, .before, .run_if ...*/
     draw_tetros,
     clear_moving_blocks.before(draw_tetros),
     apply_gravity) 
  );
  
  // TODO spawn_grid_background also when window is resized
  // RESOURCES
  app.insert_resource(Grid::new());
  app.insert_resource(GravityTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
  
  // EVENTS
  app.add_event::<NeedTetroEvent>();

  // OBSERVERS
  app.add_observer(spawn_tetro); // On NeedTetroEvent
  
  app.run();
  debug!("TETRINET END");
}
