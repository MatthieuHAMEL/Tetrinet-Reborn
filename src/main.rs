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
  grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],
  // Dimensions in world unit:
  block_sz: f32,
  width_sz: f32,
  height_sz: f32,
}

impl Grid {
  pub fn new() -> Self {
    const DEFAULT_BLOCK_SZ : f32 = 26.0;
    let grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
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
struct GridPosition {
  col: usize,
  row: usize
}

// Each block is a sprite + a GridPosition + a marker ActiveBlock
// They do not need to know their parent.

#[derive(Component)]
struct PassiveBlock;

#[derive(Event)]
struct NeedTetroEvent;

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
  ));
}

// Helper to get the real transform, function of the block pos and grid characteristics
fn block_transform(col: usize, row: usize, grid: &Grid) -> Transform {
  let x = col as f32 * grid.block_sz + grid.block_sz / 2.0 - grid.width_sz / 2.0;
  let y = -(row as f32 * grid.block_sz + grid.block_sz / 2.0) + grid.height_sz / 2.0;

  debug!("block_transform -> abs={},ord={}", x, y);
  Transform::from_xyz(x, y, 0.0)
}

// On NeedTetroEvent
fn spawn_tetro(
  _trigger: Trigger<NeedTetroEvent>,
  mut commands: Commands,
  grid_zone: Single<Entity, With<GridZoneMarker>>,
  grid: Res<Grid>) // TODO NO
{
  // Get the middle coordinate of the grid
  let middle: usize = GRID_WIDTH / 2; // TODO wrong 
  debug!("Spawning tetro at column {}", middle);

  let coos = [(middle-1, 0), (middle, 0), (middle+1, 0), (middle, 1)];

  let tetro_id = commands.spawn((
    Tetro { coos, kind: TetroKind::T, config: TetroRot::Base },
    ChildOf(*grid_zone),
    Transform::default(),
    Visibility::default()
  )).id();  
  
  // Spawn the corresponding sprites as children of the Tetro
  for &(col, row) in coos.iter() {
    commands.spawn((
      Sprite {
        color: LinearRgba::GREEN.into(),
        custom_size: Some(Vec2::splat(grid.block_sz)), // TODO, block_sz doesn't have anything to do in grid ! 
        ..Default::default()
      },
      block_transform(col, row, &grid), // IDEM !!! 
      ActiveBlock,
      GridPosition {col, row},
      ChildOf(tetro_id),
    ));
  }
  // For now just I spawn a T block
  // TODO create a helper function with default pos for all types of blocks
  // TODO 2 : here I could check that my new tetro's coordinates don't overlap with the grid

  
}

// HELPER
fn tetro_can_move_down(tetro: &Tetro, grid: &Grid) -> bool {
  tetro.coos.iter().all(|coord| {
    if coord.1 == GRID_HEIGHT - 1 { // floor -> can't move down!
      return false;
    }
    debug!("looking at grid[{}][{}]", coord.1 + 1, coord.0);
    !grid.grid[coord.1 + 1][coord.0]
  })
}

// Grid is filled only when we LOCK the tetromino
fn apply_gravity(
  mut commands: Commands,
  mut tetros: Query<(Entity, &mut Tetro, &Children)>,
  mut blocks: Query<(&mut GridPosition, &mut Transform), With<ActiveBlock>>,
  mut grid: ResMut<Grid>,
  grid_zone: Single<Entity, With<GridZoneMarker>>,
  mut gravity_timer: ResMut<GravityTimer>, time: Res<Time>)
{
  gravity_timer.0.tick(time.delta());
  if !gravity_timer.0.finished() {
    return;
  }
  
  for (entity, mut tetro, children) in tetros.iter_mut() {
    if tetro_can_move_down(&tetro, &grid) {
      // Move down
      for coord in tetro.coos.iter_mut() {
        coord.1 += 1;
      }

      // Update the children blocks GridPosition and Transform
      for (child, &(col, row)) in children.iter().zip(tetro.coos.iter()) {
        if let Ok((mut grid_pos, mut transform)) = blocks.get_mut(child) {
          grid_pos.col = col;
          grid_pos.row = row;
          *transform = block_transform(col, row, &grid); // update visual pos
        }
      }
    } else {
      // lock in grid and despawn tetro
      lock_tetro(&mut commands, &children, &mut blocks, &mut grid, &grid_zone);
      
      // Despawn the tetro. Sprites / blocks have been reparented in lock_tetro.
      commands.entity(entity).despawn();
      // Emit event to spawn a new tetro
      commands.trigger(NeedTetroEvent);
      // Time to update the grid (TODO)
    } // TODO -> blocks (sprites) in the grid shouldn't be ActiveBlocks anymore !
  }
}

fn lock_tetro(
  commands: &mut Commands,
  children: &Children,
  block_query: &mut Query<(&mut GridPosition, &mut Transform), With<ActiveBlock>>,
  grid: &mut Grid,
  grid_zone: &Single<Entity, With<GridZoneMarker>>)
{
    for block_entity in children.iter() {
      if let Ok((grid_pos, _)) = block_query.get_mut(block_entity) {
        // Update the grid data
        grid.grid[grid_pos.row][grid_pos.col] = true;
        // Remove ActiveBlock, add PassiveBlock
        commands.entity(block_entity)
          .remove::<ActiveBlock>()
          .insert(PassiveBlock); // Maybe not needed ? TODO 
        // Detach from tetromino parent (or re-parent to the grid root)
        commands.entity(block_entity).remove::<ChildOf>().insert(ChildOf(**grid_zone));
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
  app.add_systems(Update, apply_gravity); // with a timer 
  
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
