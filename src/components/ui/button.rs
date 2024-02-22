use crate::utils::build_classes;
use leptos::*;

#[component]
pub fn Button(
    #[prop(default = true)] variant_default: bool,
    #[prop(optional, into)] variant_destructive: bool,
    #[prop(optional, into)] variant_outline: bool,
    #[prop(optional, into)] variant_secondary: bool,
    #[prop(optional, into)] variant_ghost: bool,
    #[prop(optional, into)] variant_link: bool,

    #[prop(default = true)] size_default: bool,
    #[prop(optional, into)] size_sm: bool,
    #[prop(optional, into)] size_lg: bool,

    #[prop(optional)] class: &'static str,
    #[prop(optional)] disabled: bool,

    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,

    children: Children,
) -> impl IntoView {
    let classes = build_classes(vec![
        Some("inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"),

        variant_default.then_some("bg-primary text-primary-foreground hover:bg-primary/90"),
        variant_destructive.then_some("bg-destructive text-destructive-foreground hover:bg-destructive/90"),
        variant_outline.then_some("border border-input bg-background hover:bg-accent hover:text-accent-foreground"),
        variant_secondary.then_some("bg-secondary text-secondary-foreground hover:bg-secondary/80"),
        variant_ghost.then_some("hover:bg-accent hover:text-accent-foreground"),
        variant_link.then_some("text-primary underline-offset-4 hover:underline"),

        size_default.then_some("h-10 px-4 py-2"),
        size_sm.then_some("h-9 rounded-md px-3"),
        size_lg.then_some("h-11 rounded-md px-8"),

        Some(class)
    ]);

    let on_click = move |event| {
        let Some(callback) = on_click.as_ref() else {
            return;
        };
        callback.call(event);
    };

    view! {
        <button disabled=disabled class=classes on:click=on_click>{children()}</button>
    }
}
