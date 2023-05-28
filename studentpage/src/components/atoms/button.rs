use yew::prelude::*;

/// Variants of the [Button]
#[allow(dead_code)] // TODO: remove if some variants never get used
#[derive(PartialEq)]
pub enum ButtonVariant {
    Primary,
    Warning,
    Success,
    Danger,
    Info,
    Light,
}

/// Properties for [Button]
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The text to display on the button.
    pub label: AttrValue,
    /// Any additional classes to apply to the button.
    #[prop_or_default]
    pub class: AttrValue,
    /// The variant of the button.
    #[prop_or(ButtonVariant::Light)]
    pub variant: ButtonVariant,
    /// The onclick callback.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let get_variant = |variant: &ButtonVariant| match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Warning => "btn-warning",
        ButtonVariant::Success => "btn-success",
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Info => "btn-info",
        ButtonVariant::Light => "btn-light",
    };

    html! {
        <button
            class={format!("btn shadow {} {}", get_variant(&props.variant), props.class)}
            onclick={ &props.onclick }
            style="background-image: linear-gradient(135deg, #FFFFFF20, #00000020)">
            { &*props.label }
        </button>
    }
}
