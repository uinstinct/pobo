use leptos::*;

mod state;
mod timer_input;

use state::{SessionComponentEnum, SessionComponentState};
use timer_input::TimerInput;

#[component]
pub fn Session() -> impl IntoView {
    let session_component_state = SessionComponentState::new();

    provide_context(session_component_state);

    view! {
        <div class="h-screen flex justify-center items-center">
            {move || match session_component_state.component.get() {
                SessionComponentEnum::Timerinput => {
                    view! {<TimerInput />}.into_view()
                },
                SessionComponentEnum::Timer => {
                    view! {<h1>timer</h1>}.into_view()
                },
                SessionComponentEnum::Stopwatch => {
                    view! {<h1>stopwatch</h1>}.into_view()
                }
            }}
        </div>
    }
}
