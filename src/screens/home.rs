use crate::components::ui::Button;
use crate::screens::state::{AppComponentEnum, AppComponentState};
use leptos::*;

const START_SESSION: &str = "Start Session";
const QUICK_SESSION: &str = "Quick Session";

#[component]
pub fn Home() -> impl IntoView {
    let app_component_state =
        use_context::<AppComponentState>().expect("AppComponentState not provided");

    view! {
        <button class="absolute top-2 right-2">
        <img src="/public/icons/settings.svg" className="h-6 w-h-6" />
      </button>
      <div class="h-screen flex justify-center items-center flex-col">
        <Button size_lg=true on_click=move |_| app_component_state.component.set(AppComponentEnum::Session)>{START_SESSION}</Button>
        <Button variant_secondary=true class="mt-2 rounded-3xl">{QUICK_SESSION}</Button>
      </div>
    }
}
