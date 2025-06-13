use bevy::{input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion}, prelude::*, window::{CursorGrabMode, PrimaryWindow, WindowFocused}};

// May be used later 
enum CellColor { Blue, Cyan, Green, Magenta, Purple, Red, Yellow }

#[derive(Copy, Clone)]
struct Cell {
  color: Color
}

const GRID_HEIGHT: usize = 20;
const GRID_WIDTH: usize = 10;
const BLOCK_SIZE: f32 = 24.0; // in world unit 

#[derive(Resource)]
struct Grid {
  grid: [[Option<Cell>; GRID_WIDTH]; GRID_HEIGHT],
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

fn main() {
  println!("TETRINET BEGIN");

  let mut app = App::new();
  app.add_plugins(DefaultPlugins);

  // systems (functions) that run only at startup
  // app.add_systems(Startup, ());

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
