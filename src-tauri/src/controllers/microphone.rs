use log::debug;

use tauri::AppHandle;

use crate::models::errors::MICROPHONE_ERROR;
use crate::models::*;

type CommandResult<T> = std::result::Result<T, String>;

#[tauri::command]
pub fn start_microphone(app: AppHandle, state: tauri::State<State>) -> CommandResult<()> {
    debug!("Starting microphone recording");
    let input_device = AudioDevices::get_default_input_device().map_err(|_| MICROPHONE_ERROR)?;
    let input_device_config =
        AudioDevices::get_default_config(&input_device).map_err(|_| MICROPHONE_ERROR)?;
    state.is_recording.store(true); //
    let new_thread = Microphone::start(
        input_device,
        input_device_config,
        state.is_recording.clone(),
        app,
    );
    state.microphone_thread.store(Some(new_thread));
    Ok(())
}

#[tauri::command]
pub fn stop_microphone(state: tauri::State<State>) {
    if !(state.is_recording.load()) {
        return;
    }
    state.is_recording.store(false);
    let thread = state.microphone_thread.take();
    if let Some(thread) = thread {
        thread.join().unwrap().unwrap();
    }
}
