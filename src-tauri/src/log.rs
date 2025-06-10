use tauri::{plugin::TauriPlugin, Runtime};
pub use tracing::*;

pub fn init_tauri_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    // use tauri_plugin_log::{Builder, Target, TargetKind::Stdout};
    // Builder::new().target(Target::new(Stdout)).build()
    tauri_plugin_log::Builder::new().build()
}

pub fn init_tracing_subscriber_log() {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init()
}
