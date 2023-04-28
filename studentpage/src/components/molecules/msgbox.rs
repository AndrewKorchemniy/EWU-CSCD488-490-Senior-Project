use yew::prelude::*;

/// Variants of the [Message]
#[derive(PartialEq, Clone)]
pub enum MsgBoxVariant {
    Success,
    Info,
    Warning,
    Danger,
}

/// Properties for [Message]
#[derive(Properties, PartialEq)]
pub struct Props {
    pub variant: MsgBoxVariant,
    pub title: String,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MsgBox)]
pub fn msg(props: &Props) -> Html {
    let get_variant = |variant: &MsgBoxVariant| match variant {
        MsgBoxVariant::Success => "text-success border-success",
        MsgBoxVariant::Info => "text-info border-info",
        MsgBoxVariant::Warning => "text-warning border-warning",
        MsgBoxVariant::Danger => "text-danger border-danger",
    };

    html! {
        <div class={format!("card shadow {}", get_variant(&props.variant))}>
            <div class="card-body">
                <h5 class="card-title"> { props.title.clone() } </h5>
                <p class="card-text"> { props.text.clone() } </p>
                { for props.children.iter() }
            </div>
        </div>
    }
}
