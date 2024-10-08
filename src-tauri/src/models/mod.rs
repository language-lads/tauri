pub mod audio_devices;
pub mod settings;
pub mod state;
pub mod microphone;

pub use audio_devices::*;
pub use settings::*;
pub use state::*;
pub use microphone::*;

pub type Thread = std::thread::JoinHandle<Result<()>>;
pub type Threads = Vec<Thread>;

/// Timestamp in decimal seconds, e.g. 161576.123
pub type Timestamp = f64;

// Expand the `Result` type to include `anyhow::Error`
pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub type ListeningState = std::sync::Arc<crossbeam::atomic::AtomicCell<bool>>;
