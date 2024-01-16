use crate::screens::state::{AppComponentEnum, AppComponentState, ThemeState};
use crate::screens::Home;
use crate::screens::Session;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let app_component_state = AppComponentState::new();
    let theme_state = ThemeState::new();

    provide_context(app_component_state);
    provide_context(theme_state);

    create_effect(move |_| theme_state.set_document_theme());

    // TODO: implement internationalization props here

    view! {
        <Suspense>
            {move || match app_component_state.component.get() {
                AppComponentEnum::Home => {
                    view! {<Home/>}
                },
                AppComponentEnum::Session => {
                    view! {<Session />}
                }
            }}
        </Suspense>
    }
}
