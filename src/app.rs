use crate::tauri;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = tauri::invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <div class="p-6 text-xl">
                something present
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <p>
                "Recommended IDE setup: "
                <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
                " + "
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
                " + "
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
            </p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
