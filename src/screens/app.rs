use crate::screens::home::Home;
use crate::state::ThemeState;
use leptos::*;
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
        <Suspense>
            <div class="h-screen flex justify-center items-center">
                <Home />
            </div>
        </Suspense>
    }
}
