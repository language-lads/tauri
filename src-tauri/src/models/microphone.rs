use crossbeam::channel::Sender;
use std::sync::Arc;

use crate::models::*;
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::InputCallbackInfo;
use crossbeam::atomic::AtomicCell;
use log::error;
use rubato::{FftFixedIn, Resampler};

pub struct Microphone {}

impl Microphone {
    pub fn start(
        input_device: cpal::Device,
        input_device_config: cpal::StreamConfig,
        stop_flag: Arc<AtomicCell<bool>>,
        audio_samples_senders: Vec<Sender<Audio>>,
    ) -> Thread {
        std::thread::spawn(move || {
            let stop_flag_arc = stop_flag.clone();
            // Create a stream thread to capture audio from the input device
            let audio_stream = input_device.build_input_stream(
                &input_device_config,
                move |data: &[f32], _: &InputCallbackInfo| {
                    let data = data.to_vec();

                    // We need to resample the incoming audio to 24kHz
                    let mut resampler = FftFixedIn::<f32>::new(
                        input_device_config.sample_rate.0 as usize, // input sample rate
                        input_device_config.sample_rate.0 as usize, // output sample rate
                        data.len(),                                 // input buffer size
                        data.len(),                                 // sub chunks
                        input_device_config.channels.into(),        // channels
                    );

                    let resampled_data = &resampler
                        .process(&[data.clone()])
                        .expect("Failed to resample audio")[0];

                    crate::send_to_channels![
                        audio_samples_senders,
                        resampled_data,
                        stop_flag_arc,
                        "Failed to send audio samples"
                    ];
                },
                |err| {
                    error!("The input audio stream failed: {}", err);
                },
                None,
            )?;

            audio_stream.play()?;

            // Periodically check if we're done streaming
            while stop_flag.load() {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }

            drop(audio_stream);
            Ok(())
        })
    }
}
