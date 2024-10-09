use std::sync::Arc;
use tauri::Emitter;

use crate::models::*;
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::InputCallbackInfo;
use crossbeam::atomic::AtomicCell;
use log::{error, info};
use tauri::AppHandle;

pub struct Microphone {}

impl Microphone {
    pub fn start(input_device: cpal::Device, input_device_config: cpal::StreamConfig,
        stop_flag: Arc<AtomicCell<bool>>, app: AppHandle) -> Thread {
        std::thread::spawn(move || {
            // Create a stream thread to capture audio from the input device
            let audio_stream = input_device.build_input_stream(
                &input_device_config,
                move |data: &[f32], _: &InputCallbackInfo| {
                    let data = data.to_vec();
                    info!("Received audio data");
                    app.emit("audio_data", data)
                        .expect("Failed to emit audio data");
                },
                |err| {
                    error!("The input audio stream failed: {}", err);
                },
                None,
            )?;

            // Start streaming...
            audio_stream.play()?;

            // Keep checking if we're done streaming
            while stop_flag.load() {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }

            drop(audio_stream);
            Ok(())
        })
    }
}
