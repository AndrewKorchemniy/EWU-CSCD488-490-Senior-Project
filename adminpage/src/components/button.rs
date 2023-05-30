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
    Dark,
}

/// Properties for [Button]
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Text to display within the button.
    pub label: AttrValue,
    /// The type of the button.
    #[prop_or_default]
    pub button_type: String,
    /// Additional classes.
    #[prop_or_default]
    pub class: Classes,
    /// The variant of the button.
    #[prop_or(ButtonVariant::Light)]
    pub variant: ButtonVariant,
    /// data-bs-toggle
    #[prop_or_default]
    pub data_bs_toggle: String,
    /// data-bs-target
    #[prop_or_default]
    pub data_bs_target: AttrValue,
    /// The onclick callback.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// The [Button] component provides a styled button.
/// See https://getbootstrap.com/docs/5.3/components/buttons/
#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let get_variant = |variant: &ButtonVariant| match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Warning => "btn-warning",
        ButtonVariant::Success => "btn-success",
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Info => "btn-info",
        ButtonVariant::Light => "btn-light",
        ButtonVariant::Dark => "btn-dark",
    };

    html! {
        <button
            class={classes!(
                "btn", "shadow",
                get_variant(&props.variant),
                props.class.clone(),
            )}
            data-bs-toggle={ props.data_bs_toggle.clone() }
            data-bs-target={ props.data_bs_target.clone() }
            type={ props.button_type.clone() }
            onclick={ &props.onclick }>
            { &*props.label }
        </button>
    }
}
