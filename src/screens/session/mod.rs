use futures::StreamExt;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_router::{use_navigate, Outlet, Route};
use serde::{Deserialize, Serialize};
use tauri_sys::{event::listen, tauri::invoke};

mod stopwatch;
mod timer;
mod timer_input;

#[derive(Serialize, Deserialize, Clone)]
struct ResyncTimerResult {
    elapsed: Option<u64>,
    timer_seconds: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ResyncStopwatchResult {
    elapsed: Option<u64>,
}

#[component(transparent)]
pub fn SessionRoutes() -> impl IntoView {
    // listen("stopwatch_started");
    let navigate = use_navigate();

    let resync_timer_resource = create_resource(
        || (),
        |_| async move {
            let invoke_result = invoke::<_, ResyncTimerResult>("resync_timer", &()).await;
            if let Ok(invoke_result) = invoke_result {
                invoke_result
            } else {
                ResyncTimerResult {
                    elapsed: None,
                    timer_seconds: None,
                }
            }
        },
    );

    let resync_stopwatch_resource = create_resource(
        || (),
        |_| async move {
            let invoke_result = invoke("resync_stopwatch", &()).await;
            if let Ok(invoke_result) = invoke_result {
                invoke_result
            }
            ResyncStopwatchResult { elapsed: None }
        },
    );

    let stopwatch_started_resource = create_resource(
        || (),
        |_| async move {
            let event_stream = listen::<()>("stopwatch_started").await;
            if event_stream.is_err() {
                return false;
            }
            let mut event_stream = event_stream.unwrap();
            let stopwatch_started_event = event_stream.next().await;
            if let Some(stopwatch_started_event) = stopwatch_started_event {
                console_log(format!("stopwatch started {:#?}", stopwatch_started_event).as_str());
                return true;
            }
            false
        },
    );

    create_effect(move |_| {
        if let Some(resync_timer_result) = resync_timer_resource.get() {
            if resync_timer_result.elapsed.is_some() && resync_timer_result.timer_seconds.is_some()
            {
                navigate("/session/timer", Default::default());
            }
        }

        if let Some(resync_stopwatch_result) = resync_stopwatch_resource.get() {
            if resync_stopwatch_result.elapsed.is_some() {
                navigate("/session/stopwatch", Default::default());
            }
        }

        if let Some(stopwatch_started_result) = stopwatch_started_resource.get() {
            if stopwatch_started_result {
                navigate("/session/stopwatch", Default::default());
            }
        }
    });

    view! {
        <Route path=":id" view={|| view! {
            <Outlet />
        }}>
            <Route path="timer-input" view=timer_input::TimerInput />
            <Route path="timer" view=timer::SessionTimer />
            <Route path="stopwatch" view=stopwatch::Stopwatch />
            <Route path="" view=|| {
                let navigate = use_navigate();
                navigate("/session/timer-input", Default::default());
            } />
        </Route>
    }
}
