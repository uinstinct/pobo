use leptos::{leptos_dom::logging::console_log, *};
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;

use crate::{
    components::ui::{Button, NumberInput},
    utils::log_error,
};

const SETTINGS: &'static str = "Settings";
const COOLDOWN_PERIOD: &'static str = "Cooldown Period";
const COOLDOWN_PERIOD_DESCRIPTION: &'static str = "Sends a notification when cooldown finishes";
const SAVE_SETTINGS: &'static str = "Save";
const SECONDS: &'static str = "Seconds";

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
fn FormField(
    label: String,
    #[prop(default="".to_string())] description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="rounded-lg border mx-2 p-4 grid grid-cols-1 gap-2 sm:grid-cols-4">
            <div class="col-span-3 flex flex-col items-center space-y-0.5 sm:items-start">
                <label class="text-base text-center sm:text-start inline-block">{label}</label>
                <p class="text-sm text-muted-foreground text-center sm:text-start">{description}</p>
            </div>
            {children()}
        </div>
    }
}

#[component]
pub fn Settings() -> impl IntoView {
    let stopwatch_seconds = create_rw_signal::<i64>(0);

    let fetch_settings = move || async move {
        let invoke_result = invoke::<_, StoredSettingsResult>("resync_settings", &()).await;
        if let Ok(invoke_result) = invoke_result {
            stopwatch_seconds.set(invoke_result.stopwatch_seconds as i64);
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
            if invoke_result.is_ok() {
                console_log("saved!");
            } else {
                console_log(invoke_result.unwrap_err().to_string().as_str());
            }
        }
    });

    let on_settings_save = move |_| {
        save_settings_action.dispatch(StoredSettingsAction {
            storedSettings: StoredSettingsResult {
                stopwatch_seconds: stopwatch_seconds.get_untracked() as u64,
            },
        })
    };

    view! {
        <Await
            future=fetch_settings
            let:_data
        >
            <a class="absolute top-2 left-2" href="/">
                <img src="/public/icons/home.svg" className="h-6 w-6" />
            </a>
            <Button class="fixed bottom-4 right-4" on_click=on_settings_save>{SAVE_SETTINGS}</Button>

            <div class="mx-auto my-4 grid gap-2 sm:w-[75vw] xl:w-[50vw]">
                <h1 class="mb-2 text-xl text-center uppercase inline-block">{SETTINGS}</h1>

                <FormField label={COOLDOWN_PERIOD.to_string()} description={COOLDOWN_PERIOD_DESCRIPTION.to_string()}>
                    <div class="flex rounded-lg shadow-sm">
                        <NumberInput class="border-e-0" state=stopwatch_seconds />
                        <label class="px-2 text-sm text-muted-foreground h-10 inline-flex items-center rounded-e-md border border-s-0">{SECONDS}</label>
                    </div>
                </FormField>
            </div>
        </Await>
    }
}
