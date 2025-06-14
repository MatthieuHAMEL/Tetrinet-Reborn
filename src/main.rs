use bevy::{input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion}, prelude::*, window::{CursorGrabMode, PrimaryWindow, WindowFocused}};

// May be used later 
enum CellColor { Blue, Cyan, Green, Magenta, Purple, Red, Yellow }

#[derive(Copy, Clone)]
struct Cell {
  color: Color
}

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
    Self {
      grid: [[None; GRID_WIDTH]; GRID_HEIGHT],
      block_sz: DEFAULT_BLOCK_SZ,
      width_sz: DEFAULT_BLOCK_SZ * GRID_WIDTH as f32,
      height_sz: DEFAULT_BLOCK_SZ * GRID_HEIGHT as f32 
    }
  }
}

// A tetromino is composed of 4 grid coordinates
struct GridCoordinate {
  x: usize,
  y: usize
}

enum TetroKind { T, L, J, S, Z, O, I }

enum TetroRot { Base, R1, R2, R3 }

// A collection of square bricks, forming a shape like L or T
#[derive(Component)]
struct Tetro {
  coos: [GridCoordinate; 4],
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
fn spawn_grid_background(
  mut commands: Commands,
  window: Single<&Window, With<PrimaryWindow>>,
  grid: Res<Grid>) -> Entity {
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
    Name::new("GridRoot"),
  )).id()
}

fn draw_grid(
  mut commands: Commands,
  grid_root: Entity, // The entity returned by spawn_grid_root
  grid: Res<Grid>,
){
  for row in 0..GRID_HEIGHT {
    for col in 0..GRID_WIDTH {
      if let Some(cell) = &grid.grid[row][col] {
        // Local position relative to grid root, origin is top-left
        let x = col as f32 * grid.block_sz + grid.block_sz / 2.0 - grid.width_sz / 2.0;
        let y = -(row as f32 * grid.block_sz + grid.block_sz / 2.0) + grid.height_sz / 2.0;
        
        commands.entity(grid_root).with_children(|parent| {
          parent.spawn((
            Sprite {
              color: cell.color,
              custom_size: Some(Vec2::splat(grid.block_sz)),
              ..Default::default()
            },
            Transform::from_xyz(x, y, 0.0),
            Name::new(format!("Block ({},{})", row, col)),
          ));
        });
      }
    }
  }
}

fn main() {
  println!("TETRINET BEGIN");

  let mut app = App::new();
  app.add_plugins(DefaultPlugins);

  // systems (functions) that run only at startup
  app.add_systems(Startup, (spawn_camera, spawn_grid_background));

  // systems that run once per frame
  //app.add_systems(Update, 
  //  ( /* systems with .after, .before, .run_if ...*/   )
//  );

  // FixedUpdate are for fixed-time deltas updates whatever the FPS is
 // app.insert_resource(Time::<Fixed>::from_hz(30.)); // default would be 60
 // app.insert_resource(Power {
 //   charging: false, 
 //   current: 0.
 // });

 // app.add_systems(FixedUpdate, 
  //  (apply_velocity,
  //  apply_gravity.before(apply_velocity),
   // bounce.after(apply_velocity))
  //);

  //app.add_event::<BallSpawn>();
  //app.add_observer(apply_grab);

  app.insert_resource(Grid {
    grid: [[None; GRID_WIDTH]; GRID_HEIGHT],
  });
  
  app.run();
  println!("TETRINET END");
}
