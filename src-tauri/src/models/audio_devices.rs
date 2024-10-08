use crate::models::*;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait};

pub struct AudioDevices {}

impl AudioDevices {
    pub fn get_default_input_device() -> Result<cpal::Device> {
        cpal::default_host()
            .default_input_device()
            .ok_or_else(|| anyhow!("No audio input device found"))
    }

    pub fn get_default_config(input_device: &cpal::Device) -> Result<cpal::StreamConfig> {
        // Prefer the closest sample rate to 16kHz
        let desired_sample_rate = 16000;
        let configs = Self::get_input_device_configs(input_device)?;
        if configs.is_empty() {
            bail!(
                "No configs found for input device {}",
                input_device.name().unwrap()
            );
        }
        let mut min_diff = u32::MAX;
        let mut default_config = None;
        for config in configs {
            let diff = (config.sample_rate.0 as i32 - desired_sample_rate).unsigned_abs();
            if diff < min_diff {
                min_diff = diff;
                default_config = Some(config);
            }
        }
        Ok(default_config.unwrap())
    }

    fn get_input_device_configs(device: &cpal::Device) -> Result<Vec<cpal::StreamConfig>> {
        let supported_input_configs = device
            .supported_input_configs()
            .context("Failed to get input device configs")?;

        let mut stream_configs: Vec<cpal::StreamConfig> = vec![];
        for config in supported_input_configs {
            stream_configs.push(config.with_max_sample_rate().into())
        }
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
