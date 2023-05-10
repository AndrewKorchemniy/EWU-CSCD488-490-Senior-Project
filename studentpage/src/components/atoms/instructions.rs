use yew::prelude::*;

/// Properties for [Instructions] component.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The text to display as HTML - wrapped in a div.
    pub text: AttrValue,
}

/// The [Instructions] component provides a styled wrapper for instructions in forms.
#[function_component(Instructions)]
pub fn instructions(props: &Props) -> Html {
    html! {
        <div class="card-subtitle">
            { Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>", props.text))) }
        </div>
    }
}
