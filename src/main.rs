mod components;
mod screens;
mod state;
mod tauri;
mod utils;

use crate::screens::app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
