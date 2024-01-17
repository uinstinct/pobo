use leptos::*;

use crate::components::timer::Timer;
use crate::components::ui::Button;

use super::{SessionComponentEnum, SessionComponentState};

const STOP_SESSION: &str = "Stop Session";

#[component]
pub fn SessionTimer() -> impl IntoView {
    let session_component_state =
        use_context::<SessionComponentState>().expect("SessionComponentState not provided");

    let manually_stop_timer = move |_| {
        session_component_state
            .component
            .set(SessionComponentEnum::Stopwatch);
    };

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
