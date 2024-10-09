#![allow(unexpected_cfgs)]
use crate::models::*;
use anyhow::anyhow;
use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::BufferSize;
use cpal::StreamConfig;
use cpal::SupportedBufferSize;
use log::debug;
use log::info;

pub struct AudioDevices {}

impl AudioDevices {
    /// Get the default audio input device for the system.
    /// Returns an error if no audio input device can be found.
    pub fn get_default_input_device() -> Result<cpal::Device> {
        if cfg!(break_audio_input_device) {
            return Err(anyhow!("Audio disabled"));
        }
        let device = cpal::default_host()
            .default_input_device()
            .ok_or(anyhow!("No audio input device found"))?;
        let device_name = device.name().context("Failed to get input device name")?;
        info!("Default audio input device: {:?}", device_name);
        Ok(device)
    }

    /// Get the default audio input device configuration for the system.
    /// Prefer a sample rate close to 24kHz, because that seems to be what
    /// all the fancy AI audio models expect.
    ///
    /// # Arguments
    ///
    /// * `input_device` - The audio device to get the default configuration for
    ///
    /// # Errors
    ///
    /// Returns an error if no configurations can be found for the input device.
    pub fn get_default_config(input_device: &cpal::Device) -> Result<cpal::StreamConfig> {
        if cfg!(break_default_input_device_config) {
            return Err(anyhow!("Audio config disabled"));
        }
        let desired_sample_rate = 24000; // Hz
        let configs = Self::get_input_device_configs(input_device)?;
        let mut config = configs
            .into_iter()
            .min_by_key(|config| (config.sample_rate.0 as i32 - desired_sample_rate).unsigned_abs())
            .ok_or(anyhow!("No configs found for input device"))?;
        let device_name = input_device.name()?;
        info!("Default config for device '{}': {:?}", device_name, config);
        config.channels = 1; // We only support mono audio
        Ok(config)
    }

    /// Get a list of configurations for an audio input device.
    ///
    /// # Arguments
    ///
    /// * `device` - The audio device to get the configurations for
    ///
    /// # Errors
    ///
    /// Returns an error if the configurations cannot be retrieved for some reason
    fn get_input_device_configs(device: &cpal::Device) -> Result<Vec<cpal::StreamConfig>> {
        let supported_input_configs = device
            .supported_input_configs()
            .context("Failed to get input device configs")?;

        let mut stream_configs: Vec<StreamConfig> = vec![];
        for config in supported_input_configs {
            let config_with_max_rate = config.clone().with_max_sample_rate();
            // Use the biggest buffer size available
            let max_buffer_size = match config_with_max_rate.buffer_size() {
                SupportedBufferSize::Range { max, .. } => Some(max),
                SupportedBufferSize::Unknown => None,
            };
            let mut config: StreamConfig = config.clone().with_max_sample_rate().into();
            if let Some(buffer_size) = max_buffer_size {
                config.buffer_size = BufferSize::Fixed(*buffer_size);
            }
            stream_configs.push(config);
        }
        let device_name = device.name()?;
        debug!(
            "Available configs for device '{}': {:?}",
            device_name, stream_configs
        );
        Ok(stream_configs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_input_device() {
        let input_audio_device = AudioDevices::get_default_input_device();
        assert!(input_audio_device.is_ok());
        println!(
            "Input audio device: {:?}",
            input_audio_device.unwrap().name().unwrap()
        );
    }

    #[test]
    fn test_input_device_configs() {
        let devices = cpal::default_host().input_devices().unwrap();
        for device in devices {
            let device_name = device.name().unwrap();
            let configs = AudioDevices::get_input_device_configs(&device).unwrap();
            let default_config = AudioDevices::get_default_config(&device).unwrap();
            println!("{} configs: ", device_name);
            println!("- (Default) {:?}", default_config);
            for config in configs {
                println!("- {:?}", config)
            }
        }
    }
}
