use tauri::AppHandle;

use crate::models::*;

#[tauri::command]
pub fn start_microphone(app: AppHandle, state: tauri::State<State>) {
    // Start the microphone
    state.is_recording.store(true);
    let microphone_thread = Microphone::start(state.is_recording.clone(), app);
    state
        .microphone_thread
        .lock()
        .expect("Could not obtain the lock for the microphone thread")
        .replace(microphone_thread);
}

#[tauri::command]
pub fn stop_microphone(state: tauri::State<State>) {
    if !(state.is_recording.load()) {
        return;
    }
    state.is_recording.store(false);
    state
        .microphone_thread
        .lock()
        .unwrap()
        .take()
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
}
