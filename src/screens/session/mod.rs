use leptos::*;

mod stopwatch;
mod timer;
mod timer_input;

use stopwatch::Stopwatch;
use timer::SessionTimer;
use timer_input::TimerInput;

#[component]
pub fn Session() -> impl IntoView {
    view! {
        <div class="h-screen flex justify-center items-center">
            <TimerInput />
        </div>
    }
}
