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

//impl Default for Settings {
//fn default() -> Self {
//    let input_device = String::from(Microphone::INPUT_DEVICE_DEFAULT_NAME);
//    let input_device_sample_rate = Microphone::get_default_sample_rate(input_device.clone());
//    let input_device_buffer_size =
//        Microphone::get_default_input_device_buffer_size(input_device.clone());
//    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();

//    Self {
//        input_device,
//        input_device_sample_rate, // in Hz
//        input_device_buffer_size,
//        context_window: 15, // seconds
//        vad_speech_threshold: 0.2,
//        vad_chunk_size: 256,           // milliseconds
//        speech_stopped_threshold: 0.1, // seconds
//        speech_to_text_language: String::from("en"),
//        openai_api_key,
//    }
//}
//}
