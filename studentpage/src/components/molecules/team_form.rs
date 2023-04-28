use yew::prelude::*;

use crate::components::atoms::button::{Button, ButtonVariant};

/// Properties for [TeamForm]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

/// The [TeamForm] component provides a styled form for the [TeamReports] page.
#[function_component(TeamForm)]
pub fn team_form(props: &Props) -> Html {
    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <form class="mt-3 row g-3">
                    { for props.children.iter() }
                    <Button variant={ ButtonVariant::Danger } label="Submit" class="mt-2 col-auto ms-2" />
                </form>
            </div>
        </div>
    }
}
