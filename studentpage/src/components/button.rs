use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub variant: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let get_variant = |variant: &str| match variant {
        "primary" => "btn-primary",
        "secondary" => "btn-secondary",
        "success" => "btn-success",
        "danger" => "btn-danger",
        "warning" => "btn-warning",
        "info" => "btn-info",
        "light" => "btn-light",
        "dark" => "btn-dark",
        _ => "btn-light",
    };

    html! {
        <button class={format!("btn shadow {} {}", get_variant(&props.variant), props.class)}>
            { &*props.label }
        </button>
    }
}
