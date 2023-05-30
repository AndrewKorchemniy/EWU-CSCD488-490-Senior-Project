use yew::prelude::*;

/// Variants of the [MsgBox]
#[allow(dead_code)] // TODO: Remove if some variants never get used.
#[derive(PartialEq, Clone)]
pub enum MsgBoxVariant {
    Success,
    Info,
    Warning,
    Secondary,
    Danger,
}

/// Properties for [MsgBox]
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The variant of the message box.
    pub variant: MsgBoxVariant,
    /// The title of the message box.
    pub title: AttrValue,
    /// The text of the message box. Defaults to an empty string.
    #[prop_or_default]
    pub text: AttrValue,
    /// Additional classes.
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

/// The [MsgBox] component provides a styled message box with four variants.
#[function_component(MsgBox)]
pub fn msg(props: &Props) -> Html {
    let get_variant = |variant: &MsgBoxVariant| match variant {
        MsgBoxVariant::Success => "text-success border-success",
        MsgBoxVariant::Info => "text-info border-info",
        MsgBoxVariant::Warning => "text-warning border-warning",
        MsgBoxVariant::Secondary => "text-secondary border-secondary",
        MsgBoxVariant::Danger => "text-danger border-danger",
    };

    html! {
        <div class={classes!(
            "card", "shadow",
            get_variant(&props.variant),
            props.class.clone(),
        )}>
            <div class="card-body">
                <h5 class="card-title"> { props.title.clone() } </h5>
                <p class="card-text"> { props.text.clone() } </p>
                { for props.children.iter() }
            </div>
        </div>
    }
}
