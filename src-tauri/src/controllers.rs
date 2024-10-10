use crossbeam::channel;
use log::{error, info, warn};
use tauri::AppHandle;
use tauri_plugin_microphone::MicrophoneExt;
use tauri_plugin_microphone::PingRequest;

use crate::models::errors::*;
use crate::models::*;

type CommandResult<T> = std::result::Result<T, String>;

#[tauri::command]
pub fn start_audio_session(state: tauri::State<State>, app: tauri::AppHandle) -> CommandResult<()> {
    if state.audio_session_active.load() {
        warn!("Attempted to start a new audio session while the previous session was active");
        stop_audio_session(state.clone());
    }
    info!("Audio session started");

    // Get audio input device settings
    let input_device = AudioDevices::get_default_input_device().map_err(|_| MICROPHONE_ERROR)?;
    let input_device_config =
        AudioDevices::get_default_config(&input_device).map_err(|_| MICROPHONE_ERROR)?;

    // Setup threadsafe channels
    let audio_samples_channel = channel::unbounded::<Audio>();

    // Start the microphone thread to record audio
    state.audio_session_active.store(true);
    let thread_handler = Microphone::start(
        input_device,
        input_device_config,
        state.audio_session_active.clone(),
        vec![audio_samples_channel.0],
    );
    state.microphone_thread.store(Some(thread_handler));

    // Start a thread to send audio to the server via a websocket
    let thread_handler = WebSocketSender::start(
        state.audio_session_active.clone(),
        audio_samples_channel.1,
        app,
    );
    state.websocket_sender_thread.store(Some(thread_handler));

    Ok(())
}

#[tauri::command]
pub fn stop_audio_session(state: tauri::State<State>) {
    if !(state.audio_session_active.load()) {
        return;
    }
    state.audio_session_active.store(false);
    let threads = [
        state.microphone_thread.take(),
        state.websocket_sender_thread.take(),
    ];
    for thread in threads.into_iter().flatten() {
        if let Err(e) = thread.join() {
            error!("Failed to join thread: {:?}", e);
        };
    }
    info!("Audio session stopped");
}

#[tauri::command]
pub fn get_permissions(app: AppHandle) {
    let ping = PingRequest {
        value: Some("hello".to_string()),
    };
    let response = app.microphone().ping(ping).unwrap();
    info!("Ping response: {:?}", response);
}
