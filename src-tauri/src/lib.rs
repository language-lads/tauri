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
        ])
        .run(tauri::generate_context!())
        .expect("Could not start the Tauri application");
}

//use cpal::traits::DeviceTrait;
//use cpal::traits::HostTrait;
//use cpal::traits::StreamTrait;
//use cpal::InputCallbackInfo;
//use cpal::StreamConfig;
//use crossbeam::atomic::AtomicCell;
//use log::error;
//use log::info;
//use std::sync::{Arc, Mutex};
//use tauri::{AppHandle, Emitter};
//use tauri_plugin_log::Target;
//use tauri_plugin_log::TargetKind;
//use tauri_plugin_microphone::*;

//pub type Thread = std::thread::JoinHandle<()>;

///// The main application state is stored in this struct
//pub struct State {
//    /// The settings for the application
//    pub transcript: Arc<Mutex<String>>,
//    pub thread: Arc<Mutex<Option<Thread>>>,
//    pub stop_flag: Arc<AtomicCell<bool>>,
//}

//pub fn get_input_audio_device_configs() -> Vec<cpal::SupportedStreamConfig> {
//    let host = cpal::default_host();
//    let input_audio_device = host
//        .default_input_device()
//        .expect("Failed to find the audio input device");
//    input_audio_device
//        .supported_input_configs()
//        .unwrap()
//        // The max and min sample rates seem to be the same usually
//        .map(|x| x.with_max_sample_rate())
//        .collect()
//}

//// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//#[tauri::command]
//fn start_microphone(app: AppHandle) {
//    println!("Microphone started");
//    let host = cpal::default_host();
//    error!("Rocky balboa");
//    let input_audio_device = host
//        .default_input_device()
//        .expect("no input device available");
//    app.emit(
//        "transcript",
//        format!(
//            "Using audio input device: {}",
//            input_audio_device.name().unwrap()
//        ),
//    )
//    .unwrap();

//    std::thread::spawn(move || {
//        // Setup the audio input
//        let host = cpal::default_host();
//        let input_audio_device = host
//            .default_input_device()
//            .expect("no input device available");
//        //info!(
//        //    "Using audio input device: {}",
//        //    input_audio_device.name().unwrap()
//        //);
//        //let input_device_config_1 = get_input_audio_device_configs()[0].clone();
//        //for config in get_input_audio_device_configs() {
//        //    info!("Audio input device config: {:?}", config);
//        //}
//        //info!(
//        //    "Using audio input device config: {:?}",
//        //    input_device_config_1
//        //);
//        //let input_device_config = cpal::StreamConfig {
//        //    channels: 2,
//        //    sample_rate: cpal::SampleRate(16000),
//        //    buffer_size: cpal::BufferSize::Default,
//        //};
//        let input_device_config: StreamConfig =
//            input_audio_device.default_input_config().unwrap().into();
//        info!("Default input config: {:?}", input_device_config);

//        //    // Create a stream thread to capture audio from the input device
//        //    let is_listening_clone = is_listening.clone();
//        let audio_stream = input_audio_device
//            .build_input_stream(
//                &input_device_config,
//                move |data: &[f32], _: &InputCallbackInfo| {
//                    let data = data.to_vec();
//                    if data.len() < 5 {
//                        return;
//                    }
//                    //println!(
//                    //    "Audio samples: {} {} {} {} {}",
//                    //    data[0], data[1], data[2], data[3], data[4]
//                    //);
//                    app.emit(
//                        "transcript",
//                        format!(
//                            "Audio samples: {} {} {} {} {}",
//                            data[0], data[1], data[2], data[3], data[4],
//                        ),
//                    )
//                    .unwrap();
//                },
//                |err| {
//                    error!("The input audio stream failed: {}", err);
//                },
//                None,
//            )
//            .expect("Failed to build input audio stream");

//        // Start streaming...
//        audio_stream.play().expect("Failed to play audio stream");

//        std::thread::sleep(std::time::Duration::from_millis(5000));
//        //// Keep checking if we're done streaming
//        //while is_listening.load() {
//        //    std::thread::sleep(std::time::Duration::from_millis(200));
//        //}

//        // All done! Let's close the stream and return
//        info!("Finished streaming from input audio device");
//        drop(audio_stream);
//        //Ok(())
//    });
//}

//#[tauri::command]
//fn start_recording(app: AppHandle, state: tauri::State<State>) {
//    let ping = PingRequest {
//        value: Some("hello".to_string()),
//    };
//    let response = app.microphone().ping(ping).unwrap();
//    info!("Ping response: {:?}", response);

//    state.transcript.lock().unwrap().clear();
//    state.transcript.lock().unwrap().push_str("DJ Khaled: ");
//    state.stop_flag.store(false);
//    let stop_flag = state.stop_flag.clone();
//    let transcript_arc = state.transcript.clone();
//    let thread = std::thread::spawn(move || {
//        // Send a transcript event every second
//        loop {
//            if stop_flag.load() {
//                break;
//            }
//            std::thread::sleep(std::time::Duration::from_millis(500));
//            let mut new_transcript = transcript_arc.lock().unwrap().clone();
//            new_transcript.push_str("another one, ");
//            transcript_arc.lock().unwrap().clear();
//            transcript_arc.lock().unwrap().push_str(&new_transcript);
//            app.emit("transcript", new_transcript).unwrap();
//        }
//    });
//    state.thread.lock().unwrap().replace(thread);
//}

//#[tauri::command]
//fn stop_recording(state: tauri::State<State>) {
//    // Stop the thread
//    state.stop_flag.store(true);
//    let thread = state.thread.lock().unwrap().take();
//    if let Some(thread) = thread {
//        thread.join().unwrap();
//    }
//}

//#[cfg_attr(mobile, tauri::mobile_entry_point)]
//pub fn run() {
//    // This state object will be shared across all our tauri controllers
//    let state = State {
//        transcript: Arc::new(Mutex::new("".to_string())),
//        thread: Arc::new(Mutex::new(None)),
//        stop_flag: Arc::new(AtomicCell::new(false)),
//    };

//    tauri::Builder::default()
//        .manage(state)
//        .plugin(tauri_plugin_shell::init())
//        .plugin(tauri_plugin_microphone::init())
//        .plugin(
//            tauri_plugin_log::Builder::new()
//                .targets([
//                    Target::new(TargetKind::Stdout),
//                    Target::new(TargetKind::LogDir { file_name: None }),
//                    Target::new(TargetKind::Webview),
//                ])
//                .build(),
//        )
//        .invoke_handler(tauri::generate_handler![
//            start_microphone,
//            start_recording,
//            stop_recording,
//        ])
//        .run(tauri::generate_context!())
//        .expect("error while running tauri application");
//}
