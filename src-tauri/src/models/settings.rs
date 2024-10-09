/// Settings for the application.
/// These are persisted in storage and loaded at startup
/// For runtime specific state, use the `State` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    /// The input audio device to use
    pub input_device: String,
    /// The sample rate in Hz for the audio input device
    pub input_device_sample_rate: u32,
    /// How many samples to read from the input audio device at a time
    pub input_device_buffer_size: usize,
}
