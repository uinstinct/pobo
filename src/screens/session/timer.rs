use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;

use crate::components::timer::Timer;
use crate::components::ui::Button;
use crate::utils::log_error;

const STOP_SESSION: &str = "Stop Session";

#[derive(Serialize, Deserialize)]
struct StopTimer {}

#[component]
pub fn SessionTimer() -> impl IntoView {
    let navigate = use_navigate();

    let invoke_stop_timer =
        create_action(|_: &()| async move { invoke::<_, ()>("stop_timer", &StopTimer {}).await });

    let manually_stop_timer = move |_| {
        invoke_stop_timer.dispatch(());
    };

    create_effect(move |_| {
        let invoke_result = invoke_stop_timer.value().get();
        if let Some(invoke_result) = invoke_result {
            if invoke_result.is_ok() {
                navigate("/session/stopwatch", Default::default());
            } else {
                log_error(invoke_result.unwrap_err());
            }
        }
    });

    view! {
        <div class="h-screen flex justify-center items-center">
            <div>
                <Timer current_secs=60 total_seconds=360  />
                <div class="flex justify-center m-5">
                    <Button
                    variant_destructive=true
                    size_lg=true
                    class="rounded-lg"
                    on_click=manually_stop_timer
                    >
                        {STOP_SESSION}
                    </Button>
                </div>
            </div>
        </div>
    }
}
