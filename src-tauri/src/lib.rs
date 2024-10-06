use crossbeam::atomic::AtomicCell;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

pub type Thread = std::thread::JoinHandle<()>;

/// The main application state is stored in this struct
pub struct State {
    /// The settings for the application
    pub transcript: Arc<Mutex<String>>,
    pub thread: Arc<Mutex<Option<Thread>>>,
    pub stop_flag: Arc<AtomicCell<bool>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_recording(app: AppHandle, state: tauri::State<State>) {
    state.transcript.lock().unwrap().clear();
    state.transcript.lock().unwrap().push_str("DJ Khaled: ");
    state.stop_flag.store(false);
    let stop_flag = state.stop_flag.clone();
    let transcript_arc = state.transcript.clone();
    let thread = std::thread::spawn(move || {
        // Send a transcript event every second
        loop {
            if stop_flag.load() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
            let mut new_transcript = transcript_arc.lock().unwrap().clone();
            new_transcript.push_str("another one, ");
            transcript_arc.lock().unwrap().clear();
            transcript_arc.lock().unwrap().push_str(&new_transcript);
            app.emit("transcript", new_transcript).unwrap();
        }
    });
    state.thread.lock().unwrap().replace(thread);
}

#[tauri::command]
fn stop_recording(state: tauri::State<State>) {
    // Stop the thread
    state.stop_flag.store(true);
    let thread = state.thread.lock().unwrap().take();
    if let Some(thread) = thread {
        thread.join().unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // This state object will be shared across all our tauri controllers
    let state = State {
        transcript: Arc::new(Mutex::new("".to_string())),
        thread: Arc::new(Mutex::new(None)),
        stop_flag: Arc::new(AtomicCell::new(false)),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_recording,
            stop_recording,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
