use rand::Rng;
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
  info!("main_menu()");
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

/** Encrypt the command: tetrisstart nickname 1.13 based on the server IP
 * Cf §IV of the specification in the notes! */
pub fn tnet_encrypt(nickname: &str, ip: [u8; 4]) -> String {
  let msg = format!("tetrisstart {} 1.13", nickname);
  let s: Vec<char> = msg.chars().collect();

  // Generate @h from IP
  let ip_hash_str = (ip[0] as u32 * 54 + ip[1] as u32 * 41 + ip[2] as u32 * 29 + ip[3] as u32 * 17).to_string();
  let h: Vec<char> = ip_hash_str.chars().collect();

  // Random salt (dec)
  let mut rng = rand::rng();
  let mut dec = rng.random_range(0..=255) as u8;
  let mut encrypted = format!("{:02X}", dec & 0xFF);

  // Encrypt loop
  for i in 0..s.len() {
    let s_char = s[i] as u8;
    let h_char = h[i % h.len()] as u8;
    dec = ((dec + s_char) % 255) as u8 ^ h_char;
    encrypted.push_str(&format!("{:02X}", dec & 0xFF));
  }

  encrypted
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
