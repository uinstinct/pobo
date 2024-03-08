use std::{fmt, str::FromStr};

use leptos::leptos_dom::logging::console_log;

pub fn build_classes(classes: Vec<Option<&str>>) -> String {
    classes
        .into_iter()
        .filter(|class| class.is_some())
        .map(|class| class.unwrap())
        // .map(|class| class.split(' ').collect::<Vec<_>>())
        // .flatten()
        // .rev()
        // .filter(|class| {
        //     let category_class = class.split('-').next().unwrap();
        //     if previous_category_classes.contains(category_class) {
        //         console_log(format!("the found category class was {}", class).as_str());
        //         return false;
        //     }
        //     previous_category_classes.insert(category_class);
        //     true
        // })
        // .rev()
        .map(|class| format!("{} {}", class.trim(), " "))
        .collect::<String>()
        .trim()
        .to_string()
}

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
