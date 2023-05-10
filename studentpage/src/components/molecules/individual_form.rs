use yew::prelude::*;

use crate::components::atoms::button::{Button, ButtonVariant};

/// Properties for [IndividualForm]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

/// The [IndividualForm] component provides a styled form wrapper.
#[function_component(IndividualForm)]
pub fn individual_form(props: &Props) -> Html {
    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <form class="mt-1 row g-3">
                    { for props.children.iter() }
                    <Button variant={ ButtonVariant::Danger } label="Submit" class="mt-3 col-auto ms-2" />
                </form>
            </div>
        </div>
    }
}
