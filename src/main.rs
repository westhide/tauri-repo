pub mod app;
pub mod rpc;

use app::*;
use leptos::prelude::*;

// #[tokio::main(flavor = "current_thread")] async
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
