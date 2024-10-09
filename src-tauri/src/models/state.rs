use std::sync::{Arc, Mutex};

use crossbeam::atomic::AtomicCell;

use crate::models::*;

/// Our running application state is stored in here.
/// It's ephemeral and will be reset when the application is closed.
/// Stuff that needs to be persisted should be kept in the `Settings` struct
pub struct State {
    /// The current settings for the application
    pub settings: Arc<Mutex<Option<Settings>>>,
    /// The thread that is running the microphone
    pub microphone_thread: Arc<AtomicCell<Option<Thread>>>,
    /// Are we currently recording from the microphone?
    pub is_recording: Arc<AtomicCell<bool>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Arc::new(Mutex::new(None)),
            microphone_thread: Arc::new(AtomicCell::new(None)),
            is_recording: Arc::new(AtomicCell::new(false)),
        }
    }
}
