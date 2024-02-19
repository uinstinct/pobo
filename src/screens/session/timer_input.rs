use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;

use crate::{
    components::ui::{Button, Input},
    utils::log_error,
};

const START_SESSION: &str = "Start Session";

fn parse_as_number(value: String) -> u32 {
    value.parse::<u32>().unwrap_or(0)
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct StartTimerInput {
    timerSeconds: u32,
}

#[component]
fn HoursMinsSecsInput(#[prop(into)] on_total_seconds_change: Callback<u32>) -> impl IntoView {
    let hours = create_rw_signal("".to_string());
    let minutes = create_rw_signal("".to_string());
    let seconds = create_rw_signal("".to_string());

    let on_time_change = move |time_signal: RwSignal<String>| {
        move |input: String| {
            let input = input.trim();
            if input.is_empty() {
                time_signal.set("".to_string());
            } else if let Ok(input) = input.parse::<u8>() {
                if input < 60 {
                    time_signal.set(input.to_string());
                } else {
                    time_signal.set(time_signal.get());
                }
            } else {
                time_signal.set(time_signal.get());
            }
            let total_seconds = (parse_as_number(hours.get()) * 60)
                + (parse_as_number(minutes.get()) * 60)
                + parse_as_number(seconds.get());

            on_total_seconds_change.call(total_seconds);
        }
    };

    view! {
        <div class="mt-6 grid grid-cols-3 gap-2">
            <Input
                placeholder="Hours"
                state=hours
                on_input=move |value| on_time_change(hours)(value)
            />
            <Input
                placeholder="Minutes"
                state=minutes
                on_input=move |value| on_time_change(minutes)(value)
            />
            <Input
                placeholder="Seconds"
                state=seconds
                on_input=move |value| on_time_change(seconds)(value)
            />
         </div>
    }
    .into_view()
}

#[component]
pub fn TimerInput() -> impl IntoView {
    let total_seconds = create_rw_signal::<u32>(0);
    let navigate = use_navigate();

    let invoke_start_timer = create_action(|timer_seconds: &u32| {
        let timerSeconds = timer_seconds.to_owned();
        async move { invoke::<_, ()>("start_timer", &StartTimerInput { timerSeconds }).await }
    });

    let on_start_session_click = move |_| {
        if total_seconds.get_untracked() == 0 {
            return;
        }
        invoke_start_timer.dispatch(total_seconds.get_untracked());
    };

    create_effect(move |_| {
        let invoke_result = invoke_start_timer.value().get();
        if let Some(invoke_result) = invoke_result {
            if invoke_result.is_ok() {
                navigate("/session/timer", Default::default());
            } else {
                log_error(invoke_result.unwrap_err());
            }
        }
    });

    view! {
        <div class="h-screen flex flex-col justify-center items-center">
           <Button on_click=on_start_session_click>{START_SESSION}</Button>
           <HoursMinsSecsInput on_total_seconds_change=move |secs| total_seconds.set(secs) />
        </div>
    }
}
