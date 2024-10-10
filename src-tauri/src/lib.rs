pub mod controllers;
pub mod macros;
pub mod models;

use controllers::*;
use models::*;
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = State::default();

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_microphone::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            start_audio_session,
            stop_audio_session,
            get_permissions,
        ])
        .run(tauri::generate_context!())
        .expect("Could not start the Tauri application");
}
