pub use tracing::*;

pub fn init_tracing_subscriber_log() {
    tracing_browser_subscriber::configure_as_global_default();
}
