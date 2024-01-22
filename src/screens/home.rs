use crate::components::ui::Button;
use leptos::*;

const START_SESSION: &str = "Start Session";
const QUICK_SESSION: &str = "Quick Session";

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <button class="absolute top-2 right-2">
        <img src="/public/icons/settings.svg" className="h-6 w-h-6" />
      </button>
      <div class="h-screen flex justify-center items-center flex-col">
        <Button size_lg=true>{START_SESSION}</Button>
        <Button variant_secondary=true class="mt-2 rounded-3xl">{QUICK_SESSION}</Button>
      </div>
    }
}
