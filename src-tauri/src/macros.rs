/// Send a value to a vector of crossbeam channels.
///
/// If any fail to send:
/// - If we've stopped listening then who cares
/// - If we haven't stopped listening then log an error
#[macro_export]
macro_rules! send_to_channels {
    ($channels:expr, $value:expr, $is_listening:expr, $error_message:expr) => {
        // Go through each channel
        for channel in $channels.iter() {
            // Try send the value
            if let Err(err) = channel.try_send($value.clone()) {
                if $is_listening.load() {
                    // If there's an error with sending AND we're still listening,
                    // it's an error worth logging
                    log::error!("{}: {}", $error_message, err);
                };
            };
        }
    };
}
