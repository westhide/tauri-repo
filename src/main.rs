pub mod app;
pub mod log;
pub mod rpc;
pub mod rpc_client;

use app::*;
use leptos::prelude::*;

// #[tokio::main(flavor = "current_thread")] async
fn main() {
    console_error_panic_hook::set_once();
    log::init_tracing_subscriber_log();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
