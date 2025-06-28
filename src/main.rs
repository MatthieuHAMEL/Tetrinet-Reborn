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

fn main_menu() {
  debug!("main_menu()");
}

fn titlescreen_ui(mut contexts: EguiContexts, mut next_state: ResMut<NextState<AppState>>) {
  // Centered at the middle of the screen
  egui::Area::new(egui::Id::new("tnet_menu"))
    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Centered with no offset
    .interactable(true)
    .show(contexts.ctx_mut(), |ui| {
      egui::Frame::new()
        .show(ui, |ui| {
          ui.vertical_centered(|ui| {
            ui.heading("This is TetriNET REBORN!");
            ui.add_space(10.0);
            if ui.button("CONNECT").clicked() {
              info!("Connect button clicked");
              next_state.set(AppState::Connecting);
            }
          });
        });
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
      }))
    .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
    // SYSTEMS
    .add_systems(EguiContextPass, titlescreen_ui.run_if(in_state(AppState::MainMenu)))
    .add_systems(OnEnter(AppState::MainMenu), main_menu);
  
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

  // TODO why does it panic at exit? cf bevy discord 
  
  app.run();
  debug!("TETRINET END");
}
