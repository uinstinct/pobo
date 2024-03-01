use futures::StreamExt;
use leptos::{leptos_dom::logging::console_log, *};
use serde::{Deserialize, Serialize};
use tauri_sys::{event::listen, tauri::invoke};

use crate::utils::log_error;

const SESSION_LABEL: &'static str = "Session : ";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetSessionCounterResult {
    counter: u64,
}

#[component]
pub fn SessionCounter() -> impl IntoView {
    let counter_value = create_rw_signal::<u64>(0);

    let fetch_session_counter = move || async move {
        let session_counter_result =
            invoke::<_, GetSessionCounterResult>("resync_session_counter", &()).await;

        if let Ok(session_counter_result) = session_counter_result {
            counter_value.set(session_counter_result.counter)
        } else {
            log_error("Session Counter should have been set!");
        }
    };

    spawn_local(async move {
        let event_stream = listen::<GetSessionCounterResult>("get_session_counter").await;
        if event_stream.is_err() {
            counter_value.set(0);
            console_log("get_session_counter gave error");
            return;
        }
        let mut event_stream = event_stream.unwrap();
        while let Some(session_counter_event) = event_stream.next().await {
            counter_value.set(session_counter_event.payload.counter);
        }
    });

    view! {
        <Await
            future=fetch_session_counter
            let:_data
        >
            <div class="absolute left-1/2 -translate-x-1/2 mt-2">{SESSION_LABEL}{counter_value}</div>
        </Await>
    }
}
