use crate::screens::state::ThemeState;
use crate::screens::{session::SessionRoutes, settings::Settings, Home};
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
            <main class="relative">
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/settings" view=Settings />
                    <SessionRoutes />
                </Routes>
            </main>
        </Router>
    }
}
