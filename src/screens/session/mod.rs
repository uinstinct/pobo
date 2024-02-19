use leptos::*;
use leptos_router::{use_navigate, Outlet, Route};

mod stopwatch;
mod timer;
mod timer_input;

#[component(transparent)]
pub fn SessionRoutes() -> impl IntoView {
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
