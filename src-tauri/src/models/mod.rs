pub mod audio_devices;
pub mod errors;
pub mod microphone;
pub mod settings;
pub mod state;
pub mod websocket_sender;

pub use audio_devices::*;
pub use errors::*;
pub use microphone::*;
pub use settings::*;
pub use state::*;
pub use websocket_sender::*;

pub type Thread = std::thread::JoinHandle<Result<()>>;
pub type Threads = Vec<Thread>;

/// Expands the `std::result::Result` type to include `anyhow::Error`
pub type Result<T> = std::result::Result<T, anyhow::Error>;

/// Floating point number representing an audio sample
pub type AudioSample = f32;

/// A collection of audio samples and their (sometimes) inferred timestamps
/// Earlier samples are at the *start* of the vector
pub type Audio = Vec<AudioSample>;
