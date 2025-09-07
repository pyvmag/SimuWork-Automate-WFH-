#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod input_simulator;
mod evasion_engine;
mod os_integration;

use evasion_engine::{Engine, ActivityLevel};
use std::sync::Mutex;
use tauri::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Coords {
  x: i32,
  y: i32,
}

struct AppState(Mutex<Engine>);

// Updated command to accept mouseSpeed
#[tauri::command]
fn start_simulation(level: ActivityLevel, coords: Coords, mouse_speed: u64, state: State<AppState>) {
    let mut engine = state.0.lock().unwrap();
    engine.set_target_coords(coords.x, coords.y);
    engine.set_mouse_speed(mouse_speed); // Set the mouse speed
    engine.run(level);
}

#[tauri::command]
fn stop_simulation(state: State<AppState>) {
    let engine = state.0.lock().unwrap();
    engine.stop();
}

fn main() {
  let initial_state = AppState(Mutex::new(Engine::new()));
  tauri::Builder::default()
    .manage(initial_state)
    .invoke_handler(tauri::generate_handler![
        start_simulation,
        stop_simulation
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}