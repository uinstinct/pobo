use leptos::{leptos_dom::logging::console_log, *};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum ThemeEnum {
    Dark,
    Light,
}

#[derive(Copy, Clone)]
pub struct ThemeState {
    pub theme: RwSignal<ThemeEnum>,
}

impl ThemeState {
    pub fn new() -> Self {
        // TODO: add the theme use matchmedia or localstorage
        return Self {
            theme: create_rw_signal(ThemeEnum::Dark),
        };
    }

    pub fn set_document_theme(&self) {
        let theme = self.theme.get();
        match theme {
            ThemeEnum::Dark => {
                console_log("setting the dark theme");
                window()
                    .document()
                    .unwrap()
                    .document_element()
                    .unwrap()
                    .class_list()
                    .add_1("dark")
                    .unwrap();
            }
            ThemeEnum::Light => {
                console_log("setting the light theme");
                window()
                    .document()
                    .unwrap()
                    .document_element()
                    .unwrap()
                    .class_list()
                    .add_1("light")
                    .unwrap();
            }
        }
    }
}
