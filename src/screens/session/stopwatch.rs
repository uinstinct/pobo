use std::time::Duration;

use futures::StreamExt;
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use tauri_sys::event::listen;
use tauri_sys::tauri::invoke;

use crate::components::timer::Timer;
use crate::components::ui::Button;

const NEXT_SESSION: &str = "Next Session";
const STOP_COOLDOWN: &str = "Stop Cooldown";

#[derive(Serialize, Deserialize)]
struct ResyncStopwatchResult {
    elapsed: Option<u64>,
}

#[component]
pub fn Stopwatch() -> impl IntoView {
    let elapsed = create_rw_signal(0);
    let timer_handle = create_rw_signal::<Option<IntervalHandle>>(None);

    let invoke_stop_stopwatch =
        create_action(|_: &()| async move { invoke::<_, ()>("stop_stopwatch", &()).await });

    let stopwatch_started_resource = create_resource(
        || (),
        |_| async move {
            let event_stream = listen::<()>("stopwatch_finished").await;
            if event_stream.is_err() {
                return;
            }
            let mut event_stream = event_stream.unwrap();
            let stopwatch_stopped_event = event_stream.next().await;
            if stopwatch_stopped_event.is_some() {
                console_log(
                    format!("stopwatch stopped {:#?}", stopwatch_stopped_event.unwrap()).as_str(),
                );
            }
        },
    );

    let fetch_stopwatch_data = move || async move {
        let stopwatch_result = invoke::<_, ResyncStopwatchResult>("resync_stopwatch", &()).await;

        if let Ok(stopwatch_result) = stopwatch_result {
            elapsed.set(stopwatch_result.elapsed.unwrap());
            let interval_result = set_interval_with_handle(
                move || {
                    elapsed.set(elapsed.get_untracked() + 1);
                },
                Duration::from_secs(1),
            );
            if let Ok(interval_handle) = interval_result {
                timer_handle.set(Some(interval_handle));
            }
        } else {
            console_log("Stopwatch should have been set!");
        }
    };

    on_cleanup(move || {
        if let Some(timer_handle) = timer_handle.get_untracked() {
            timer_handle.clear();
        }
    });

    view! {
        <Await
            future=fetch_stopwatch_data
            let:_data
            >
            <div class="h-screen flex justify-center items-center">
                <div class="flex flex-col justify-center items-center">
                    <Timer current_secs=elapsed.get() />
                    <div class="flex gap-2 m-5">
                        <Button variant_destructive=true on_click=move |_| {
                            invoke_stop_stopwatch.dispatch(());
                            let navigate = use_navigate();
                            if let Some(timer_handle) = timer_handle.get_untracked() {
                                timer_handle.clear();
                            }
                            navigate("/", Default::default());
                        }>{STOP_COOLDOWN}</Button>
                        <Button disabled=stopwatch_started_resource.loading().get() on_click=move |_| {
                            let navigate = use_navigate();
                            if let Some(timer_handle) = timer_handle.get_untracked() {
                                timer_handle.clear();
                            }
                            navigate("/session/timer-input", Default::default());
                        }>{NEXT_SESSION}</Button>
                    </div>
                </div>
            </div>
        </Await>
    }
}
