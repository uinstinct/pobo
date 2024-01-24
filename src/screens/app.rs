use crate::screens::state::ThemeState;
use crate::screens::{session, Home};
use leptos::*;
use leptos_router::{Route, Router, Routes};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let theme_state = ThemeState::new();

    provide_context(theme_state);

    create_effect(move |_| theme_state.set_document_theme());

    // TODO: implement internationalization props here

    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/session/timer-input" view=session::TimerInput />
                    <Route path="/session/timer" view=session::SessionTimer />
                    <Route path="/session/stopwatch" view=session::Stopwatch />
                </Routes>
            </main>
        </Router>
    }
}
