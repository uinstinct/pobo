use leptos::*;
use tailwind_fuse::*;

use crate::utils::convert_string_to_number;

#[derive(TwClass)]
#[tw(
    class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
)]
struct NumberInputVariant {}

#[component]
pub fn NumberInput(
    state: RwSignal<i64>,

    /// show blank input when the component mounts and then show number(state) on input
    #[prop(default = false)]
    clear_on_mount: bool,

    #[prop(optional)] class: &'static str,

    #[prop(default = "")] placeholder: &'static str,
    #[prop(default = 0)] min: i64,
    #[prop(default = i32::MAX.into())] max: i64,
    #[prop(optional, into)] on_input: Option<Callback<i64>>,
) -> impl IntoView {
    let has_typed = create_rw_signal(!clear_on_mount);

    let handle_input = move |input_value: String| {
        let number_input = convert_string_to_number::<i64>(input_value);
        if let Some(on_input) = on_input.as_ref() {
            on_input.call(number_input);
        } else {
            if number_input >= min && number_input <= max {
                state.set(number_input);
            } else {
                state.set(state.get_untracked());
            }
        }
    };

    let classes = NumberInputVariant::variant().with_class(class);

    view! {
        <input
            type="number"
            min=min
            max=max
            on:input=move |ev| {
                let input_value = event_target_value(&ev);
                has_typed.set(true);
                handle_input(input_value);
            }
            prop:value=move || {
                if has_typed.get() {
                    state.get().to_string()
                } else {
                    "".to_string()
                }
            }
            placeholder=placeholder
            class=classes
        />
    }
}
