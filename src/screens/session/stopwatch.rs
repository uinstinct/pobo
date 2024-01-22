use leptos::*;

use crate::components::timer::Timer;
use crate::components::ui::Button;

const NEXT_SESSION: &str = "Next Session";
const STOP_COOLDOWN: &str = "Stop Cooldown";

#[component]
pub fn Stopwatch() -> impl IntoView {
    view! {
        <div>
            <Timer current_secs=60 />
            <div class="flex justify-center m-5">
                <Button>{STOP_COOLDOWN}</Button>
                <Button>{NEXT_SESSION}</Button>
            </div>
        </div>
    }
}
