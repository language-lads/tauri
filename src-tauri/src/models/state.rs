use std::sync::{Arc, Mutex};

use crossbeam::atomic::AtomicCell;

use crate::models::*;

/// Our running application state is stored in here.
/// It's ephemeral and will be reset when the application is closed.
/// Stuff that needs to be persisted should be kept in the `Settings` struct
pub struct State {
    /// The current settings for the application
    pub settings: Arc<Mutex<Option<Settings>>>,
    /// Are we in the middle of an audio session?
    pub audio_session_active: Arc<AtomicCell<bool>>,
    /// The thread that is running the microphone
    pub microphone_thread: Arc<AtomicCell<Option<Thread>>>,
    /// The thread that sending audio data to the server via a websocket
    pub websocket_sender_thread: Arc<AtomicCell<Option<Thread>>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Arc::new(Mutex::new(None)),
            audio_session_active: Arc::new(AtomicCell::new(false)),
            microphone_thread: Arc::new(AtomicCell::new(None)),
            websocket_sender_thread: Arc::new(AtomicCell::new(None)),
        }
    }
}
