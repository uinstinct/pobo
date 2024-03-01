use self::session_counter::SessionCounter;
use futures::StreamExt;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_router::{use_navigate, Outlet, Route};
use serde::{Deserialize, Serialize};
use tauri_sys::{event::listen, tauri::invoke};

mod session_counter;
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
    let navigate = use_navigate();

    let resync_timer_resource = create_resource(
        || (),
        |_| async move {
            let invoke_result = invoke::<_, ResyncTimerResult>("resync_timer", &()).await;
            invoke_result.unwrap_or(ResyncTimerResult {
                elapsed: None,
                timer_seconds: None,
            })
        },
    );

    let resync_stopwatch_resource = create_resource(
        || (),
        |_| async move {
            let invoke_result = invoke("resync_stopwatch", &()).await;
            invoke_result.unwrap_or(ResyncStopwatchResult { elapsed: None })
        },
    );

    spawn_local(async {
        let event_stream = listen::<()>("stopwatch_started").await;
        if event_stream.is_err() {
            console_log("event stream produced error {:#?}");
            return;
        }
        let mut event_stream = event_stream.unwrap();
        while let Some(stopwatch_started_event) = event_stream.next().await {
            console_log(format!("stopwatch started {:#?}", stopwatch_started_event).as_str());
            let navigate = use_navigate();
            navigate("/session/stopwatch", Default::default());
        }
    });

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
    });

    view! {
        <Route path=":id" view={move || view! {
            <SessionCounter />
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
