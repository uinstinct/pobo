use leptos::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
)]
struct Btn {
    variant: BtnVariant,
    size: BtnSize,
}

#[derive(TwVariant)]
enum BtnSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-9 rounded-md px-3")]
    Sm,
    #[tw(class = "h-11 rounded-md px-8")]
    Lg,
}

#[derive(TwVariant)]
enum BtnVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[component]
pub fn Button(
    #[prop(optional, into)] variant_destructive: bool,
    #[prop(optional, into)] variant_outline: bool,
    #[prop(optional, into)] variant_secondary: bool,
    #[prop(optional, into)] variant_ghost: bool,
    #[prop(optional, into)] variant_link: bool,

    #[prop(optional, into)] size_sm: bool,
    #[prop(optional, into)] size_lg: bool,

    #[prop(optional)] class: &'static str,
    #[prop(optional)] disabled: bool,

    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,

    children: Children,
) -> impl IntoView {
    let button_variant = if variant_destructive {
        BtnVariant::Destructive
    } else if variant_secondary {
        BtnVariant::Secondary
    } else if variant_outline {
        BtnVariant::Outline
    } else if variant_ghost {
        BtnVariant::Ghost
    } else if variant_link {
        BtnVariant::Link
    } else {
        BtnVariant::Default
    };

    let button_size = if size_lg {
        BtnSize::Lg
    } else if size_sm {
        BtnSize::Sm
    } else {
        BtnSize::Default
    };

    let on_click = move |event| {
        let Some(callback) = on_click.as_ref() else {
            return;
        };
        callback.call(event);
    };

    let class = Btn::variant()
        .variant(button_variant)
        .size(button_size)
        .with_class(class);

    view! {
        <button disabled=disabled class=class on:click=on_click>{children()}</button>
    }
}
