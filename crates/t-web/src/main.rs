use leptos::prelude::*;
use t_web::{log::init_tracing_subscriber_log, view::Main};

// #[tokio::main(flavor = "current_thread")] async
fn main() {
    console_error_panic_hook::set_once();
    init_tracing_subscriber_log();
    mount_to_body(|| {
        view! {
            <Main/>
        }
    })
}
