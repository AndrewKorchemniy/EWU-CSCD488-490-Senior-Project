use yew::prelude::*;

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
                <form class="mt-1 row g-3" onsubmit={Callback::from(|event: SubmitEvent| {
                    event.prevent_default(); // prevent the defualt form submission behavior
                })}>
                    { for props.children.iter() }
                </form>
            </div>
        </div>
    }
}
