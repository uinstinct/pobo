use crate::components::ui::Button;
use leptos::*;
use leptos_router::use_navigate;

const START_SESSION: &str = "Start Session";
const QUICK_SESSION: &str = "Quick Session";

#[component]
pub fn Home() -> impl IntoView {
    let navigate = use_navigate();

    view! {
      <a class="absolute top-2 right-2" href="/settings">
        <img src="/public/icons/settings.svg" className="h-6 w-h-6" />
      </a>
      <div class="h-screen flex justify-center items-center flex-col">
        <Button size_lg=true on_click=move |_| navigate("/session/timer-input", Default::default())>{START_SESSION}</Button>
        <Button variant_secondary=true class="mt-2 rounded-3xl">{QUICK_SESSION}</Button>
      </div>
    }
}
