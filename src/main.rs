use bevy::{input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion}, prelude::*, window::{CursorGrabMode, PrimaryWindow, WindowFocused}};

use bevy::log::LogPlugin;

fn main() {
  let mut app = App::new();
  app.add_plugins(DefaultPlugins.set(LogPlugin {
    filter: "warn,tetrinet=debug".into(),
    level: bevy::log::Level::DEBUG,
    custom_layer: |_| None,
  }));

  debug!("TETRINET BEGIN");
  
  // SYSTEMS
  //app.add_systems(Startup, ());
  //app.add_systems(Update, ());
  
  // RESOURCES
  //app.insert_resource();
  
  // EVENTS
  //app.add_event::<MyEvent>();

  // OBSERVERS
  //app.add_observer(myObserver); // Can take a Trigger<MyEvent>
  
  app.run();
  debug!("TETRINET END");
}
