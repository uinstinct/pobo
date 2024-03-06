use leptos::*;

use crate::utils::build_classes;

/// the value is automatically set on the signal and always received from the signal
///
/// whenever `on_input` is provided, the signal `state` should be set explicitly in all cases
///
/// Controlled Input Component
#[component]
pub fn Switch(
    state: RwSignal<bool>,

    #[prop(optional)] class: &'static str,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] on_input: Option<Callback<bool>>,
) -> impl IntoView {
    let classes = build_classes(vec![
        Some("peer relative h-6 w-11 cursor-pointer rounded-full border-2 border-transparent transition-colors bg-input after:absolute"),
        Some("after:start-[2px] after:end-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-transparent after:bg-background after:transition-transform shadow-lg ring-0 after:content-['']"),
        Some("peer-checked:bg-primary peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:ring-ring peer-focus:ring-offset-background"),
        Some("peer-disabled:pointer-events-none peer-disabled:opacity-50 peer-disabled:cursor-not-allowed"),
        Some(class)
    ]);

    let on_check = move || {
        let value = !state.get_untracked();
        if let Some(on_input) = on_input.as_ref() {
            on_input.call(value);
        } else {
            state.set(value);
        }
    };

    view! {
        <input type="checkbox" disabled=disabled class="peer sr-only"
            prop:checked=move || state.get() on:input=move |_| on_check() />
        <div class=classes on:click=move |_| on_check() />
    }
}
