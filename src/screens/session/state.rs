use leptos::*;

#[derive(Clone, Copy)]
pub enum SessionComponentEnum {
    Timerinput,
    Timer,
    Stopwatch,
}

#[derive(Clone, Copy)]
pub struct SessionComponentState {
    pub component: RwSignal<SessionComponentEnum>,
}

impl SessionComponentState {
    pub fn new() -> Self {
        return Self {
            component: create_rw_signal(SessionComponentEnum::Timerinput),
        };
    }
}
