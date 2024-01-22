use leptos::*;

use crate::components::timer::Timer;
use crate::components::ui::Button;

const STOP_SESSION: &str = "Stop Session";

#[component]
pub fn SessionTimer() -> impl IntoView {
    let manually_stop_timer = move |_| {};

    view! {
        <div>
            <Timer current_secs=60 total_seconds=360  />
            <div class="flex justify-center m-5">
                <Button
                variant_destructive=true
                size_lg=true
                class="rounded-lg"
                on_click=manually_stop_timer
                >
                {STOP_SESSION}</Button>
            </div>
        </div>
    }
}
