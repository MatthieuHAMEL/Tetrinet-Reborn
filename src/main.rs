use bevy_egui::{EguiPlugin, EguiContextPass, EguiContexts, egui};
use bevy::{input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion}, prelude::*, window::{CursorGrabMode, PrimaryWindow, WindowFocused}};

use bevy::log::LogPlugin;

// States allow to condition the systems set in the scheduler.
// Here it's really the top-level state but I can define more.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum AppState {
  #[default] // Allows for init_state::<AppState>()
  MainMenu,
  Connecting,
  InGame
}

fn ui_example_system(mut contexts: EguiContexts) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    ui.label("world");
  });
}

fn main() {
  let mut app = App::new();
  app.add_plugins(
    DefaultPlugins
      .set(LogPlugin {
	filter: "warn,tetrinet=debug".into(),
	level: bevy::log::Level::DEBUG,
	custom_layer: |_| None,
      })
  ).add_plugins(EguiPlugin { enable_multipass_for_primary_context: true });

  app.add_systems(EguiContextPass, ui_example_system);
  

  debug!("TETRINET BEGIN");

  app.init_state::<AppState>();
  
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
