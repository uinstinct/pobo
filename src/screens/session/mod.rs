use leptos::*;

mod state;

use state::{SessionComponentEnum, SessionComponentState};

#[component]
pub fn Session() -> impl IntoView {
    let session_component_state = SessionComponentState::new();

    provide_context(session_component_state);

    view! {
        {move || match session_component_state.component.get() {
            SessionComponentEnum::Timerinput => {
                view! {<h1>timer input</h1>}
            },
            SessionComponentEnum::Timer => {
                view! {<h1>timer</h1>}
            },
            SessionComponentEnum::Stopwatch => {
                view! {<h1>stopwatch</h1>}
            }
        }}
    }
}
