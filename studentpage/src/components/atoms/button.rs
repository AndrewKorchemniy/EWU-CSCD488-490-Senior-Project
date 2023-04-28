use yew::prelude::*;

/// Variants of the [Button]
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Warning,
    Info,
    Light,
}

/// Properties for [Button]
#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or(ButtonVariant::Light)]
    pub variant: ButtonVariant,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let get_variant = |variant: &ButtonVariant| match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Warning => "btn-warning",
        ButtonVariant::Info => "btn-info",
        ButtonVariant::Light => "btn-light",
    };

    html! {
        <button class={format!("btn shadow {} {}", get_variant(&props.variant), props.class)}>
            { &*props.label }
        </button>
    }
}
