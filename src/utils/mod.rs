pub fn build_classes(classes: Vec<Option<&str>>) -> String {
    classes
        .into_iter()
        .filter(|class| class.is_some())
        .map(|class| format!("{} {}", class.unwrap(), " "))
        .collect::<String>()
}
