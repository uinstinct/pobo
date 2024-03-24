use std::{fmt, str::FromStr};

use leptos::leptos_dom::logging::console_log;

pub fn log_error(err: impl fmt::Debug) {
    console_log(format!("NOT EXPECTED ERROR {:#?}", err).as_str());
}

pub fn convert_string_to_number<T: FromStr + Default>(input: String) -> T {
    let input = input.trim();
    if input.is_empty() {
        Default::default()
    } else if let Ok(result_number) = input.parse::<T>() {
        result_number
    } else {
        Default::default()
    }
}
