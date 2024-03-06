use leptos::*;

use crate::utils::build_classes;

/// the value is automatically set on the signal and always received from the signal
///
/// whenever `on_input` is provided, the signal `state` should be explicitly set in all cases - otherwise the typed input and the signal's value would not match (i.e. signal should always **control** the value inside input)
///
/// Controlled Input Component
#[component]
pub fn Input(
    state: RwSignal<String>,

    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(default = "")] placeholder: &'static str,
    #[prop(optional, into)] on_input: Option<Callback<String>>,
) -> impl IntoView {
    let classes = build_classes(vec![
        Some("flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"),
        Some(class)
    ]);

    view! {
        <input
            type={input_type}
            on:input=move |ev| {
                let input_value = event_target_value(&ev);
                if let Some(on_input) = on_input.as_ref() {
                    on_input.call(input_value);
                } else {
                    state.set(input_value);
                }
            }
            prop:value=move || {
                state.get()
            }
            placeholder=placeholder
            class=classes
        />
    }
}
