use crossbeam::channel::Receiver;
use std::{sync::Arc, time::Duration};

use tauri::{AppHandle, Emitter};

use crate::models::*;
use crossbeam::atomic::AtomicCell;

pub struct WebSocketSender {}

impl WebSocketSender {
    pub fn start(
        stop_flag: Arc<AtomicCell<bool>>,
        audio_samples_receiver: Receiver<Audio>,
        app: AppHandle,
    ) -> Thread {
        std::thread::spawn(move || {
            loop {
                // Check if we're finished streaming and there's no more data to process
                if !stop_flag.load() && audio_samples_receiver.is_empty() {
                    break;
                }

                let Ok(audio_samples) =
                    audio_samples_receiver.recv_timeout(Duration::from_millis(100))
                else {
                    // If we didn't receive any audio samples, skip to the next iteration
                    // This makes sure we keep checking if we're finished or not
                    continue;
                };

                app.emit("audio_data", audio_samples)
                    .expect("Failed to emit audio data");
            }
            Ok(())
        })
    }
}
