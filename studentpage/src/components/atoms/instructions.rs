use yew::prelude::*;

/// Properties for [Instructions] component.
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

/// The [Instructions] component provides a styled instructions section within a form.
#[function_component(Instructions)]
pub fn instructions(props: &Props) -> Html {
    html! {
        <div class="card-subtitle">
            { for props.children.iter() }
            <hr/>
        </div>
    }
}
