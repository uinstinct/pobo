use leptos::{leptos_dom::logging::console_log, *};
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;

use crate::{
    components::ui::{Button, Input},
    utils::log_error,
};

const SETTINGS: &'static str = "Settings";
const COOLDOWN_PERIOD: &'static str = "Cooldown Period";
const COOLDOWN_PERIOD_DESCRIPTION: &'static str = "Sends a notification when cooldown finishes";
const SAVE_SETTINGS: &'static str = "Save";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSettingsResult {
    stopwatch_seconds: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSettingsAction {
    storedSettings: StoredSettingsResult,
}

#[component]
pub fn Settings() -> impl IntoView {
    let stopwatch_seconds = create_rw_signal(String::from("0"));

    let fetch_settings = move || async move {
        let invoke_result = invoke::<_, StoredSettingsResult>("resync_settings", &()).await;
        if let Ok(invoke_result) = invoke_result {
            stopwatch_seconds.set(invoke_result.stopwatch_seconds.to_string());
        } else {
            log_error("Settings should have been set");
        }
    };

    let save_settings_action = create_action(|stored_settings: &StoredSettingsAction| {
        let stored_settings = stored_settings.to_owned();
        async move {
            let invoke_result =
                invoke::<StoredSettingsAction, ()>("set_settings", &stored_settings).await;
            console_log("calling store_settings");
            if let Ok(_) = invoke_result {
                console_log("saved!");
            } else {
                console_log(invoke_result.unwrap_err().to_string().as_str());
            }
        }
    });

    let on_settings_save = move |_| {
        save_settings_action.dispatch(StoredSettingsAction {
            storedSettings: StoredSettingsResult {
                stopwatch_seconds: stopwatch_seconds.get().parse::<u64>().unwrap(),
            },
        })
    };

    view! {
        <Await
            future=fetch_settings
            let:_data
        >
            <a class="absolute top-2 right-2" href="/">
                <img src="/public/icons/home.svg" className="h-6 w-h-6" />
            </a>
            <Button on_click=on_settings_save>{SAVE_SETTINGS}</Button>

            <div class="mx-auto my-4 grid gap-2 sm:w-[75vw] xl:w-[50vw]">
                <h1 class="mb-2 text-center uppercase inline-block">{SETTINGS}</h1>

                <div class="flex flex-wrap items-center justify-center gap-2 rounded-lg border p-4 sm:justify-between mx-4">
                    <div class="w-full flex flex-col items-center space-y-0.5 sm:w-3/4 sm:items-start">
                        <label class="text-base inline-block">{COOLDOWN_PERIOD}</label>
                        <p class="text-sm text-muted-foreground text-center sm:text-start">{COOLDOWN_PERIOD_DESCRIPTION}</p>
                    </div>
                    <Input state=stopwatch_seconds />
                </div>
            </div>
        </Await>
    }
}
