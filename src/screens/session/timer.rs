use std::time::Duration;

use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;

use crate::components::timer::Timer;
use crate::components::ui::Button;
use crate::utils::log_error;

const STOP_SESSION: &str = "Stop Session";

#[derive(Serialize, Deserialize)]
struct ResyncTimerResult {
    elapsed: Option<u64>,
    timer_seconds: Option<u64>,
}

#[component]
pub fn SessionTimer() -> impl IntoView {
    let navigate = use_navigate();

    let elapsed = create_rw_signal(0);
    let timer_seconds = create_rw_signal(0);
    let timer_handle = create_rw_signal::<Option<IntervalHandle>>(None);

    let invoke_stop_timer =
        create_action(|_: &()| async move { invoke::<_, ()>("stop_timer", &()).await });

    let manually_stop_timer = move |_| {
        invoke_stop_timer.dispatch(());
        if let Some(timer_handle) = timer_handle.get_untracked() {
            timer_handle.clear();
        }
    };

    let fetch_timer_data = move || async move {
        let timer_result = invoke::<_, ResyncTimerResult>("resync_timer", &()).await;

        if let Ok(timer_result) = timer_result {
            elapsed.set(timer_result.elapsed.unwrap());
            timer_seconds.set_untracked(timer_result.timer_seconds.unwrap());

            let interval_result = set_interval_with_handle(
                move || {
                    elapsed.set(elapsed.get() + 1);
                },
                Duration::from_secs(1),
            );
            if let Ok(interval_handle) = interval_result {
                timer_handle.set_untracked(Some(interval_handle));
            };
        } else {
            log_error("Timer should have been set!");
        }
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
        <Await
            future=move || fetch_timer_data()
            let:_data
            >
            <div class="h-screen flex justify-center items-center">
                <div>
                    <Timer current_secs=elapsed.get() total_seconds=timer_seconds.get()  />
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
        </Await>
    }
}
