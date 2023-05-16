use yew::prelude::*;
/// Properties for [TeamForm]
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The onsubmit callback.
    pub onsubmit: Callback<SubmitEvent>,
    pub children: Children,
}

/// The [TeamForm] component provides a styled form wrapper.
#[function_component(TeamForm)]
pub fn team_form(props: &Props) -> Html {
    let onsubmit = props.onsubmit.clone();

    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <form class="mt-1 row g-3" {onsubmit}>
                    { for props.children.iter() }
                </form>
            </div>
        </div>
    }
}
