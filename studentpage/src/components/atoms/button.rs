use yew::prelude::*;

/// Variants of the [Button]
#[derive(PartialEq)]
pub enum ButtonVariant {
    Primary,
    Warning,
    Danger,
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
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Info => "btn-info",
        ButtonVariant::Light => "btn-light",
    };

    html! {
        <button class={format!("btn shadow {} {}", get_variant(&props.variant), props.class)}
            style="background-image: linear-gradient(225deg, #FFFFFF30, #00000020)">
            { &*props.label }
        </button>
    }
}
