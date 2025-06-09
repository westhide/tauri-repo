// pub use log::{debug, error, info, warn};
use tauri::{plugin::TauriPlugin, Runtime};
pub use tracing::{debug, error, info, warn};

pub fn init_tauri_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    // use tauri_plugin_log::{Builder, Target, TargetKind::Stdout};
    // Builder::new().target(Target::new(Stdout)).build()

    tauri_plugin_log::Builder::new().build()
}
