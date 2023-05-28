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
    /// Additional classes.
    #[prop_or_default]
    pub class: Classes,
    /// Variant of the button.
    #[prop_or(ButtonVariant::Light)]
    pub variant: ButtonVariant,
    /// data-bs-toggle
    #[prop_or_default]
    pub data_bs_toggle: String,
    /// Button type
    #[prop_or("button".to_string())]
    pub button_type: String,
    /// Aria state
    #[prop_or("false".to_string())]
    pub aria_expanded: String,
    /// Onclick callback
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
        ButtonVariant::Dark => "btn-dark",
    };

    html! {
        <button
            class={classes!(
                "btn", "shadow",
                get_variant(&props.variant),
                props.class.clone(),
            )}
            style="background-image: linear-gradient(135deg, #FFFFFF20, #00000020)"
            data-bs-toggle={ props.data_bs_toggle.clone() }
            aria-expanded={ props.aria_expanded.clone() }
            type={ props.button_type.clone() }
            onclick={ &props.onclick }>
            { &*props.label }
        </button>
    }
}
