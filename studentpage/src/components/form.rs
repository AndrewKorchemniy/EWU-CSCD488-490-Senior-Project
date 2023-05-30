use yew::prelude::*;

/// Properties for [Form]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

/// The [Form] component provides a styled form wrapper.
/// This uses a bootstrap card.
/// See https://getbootstrap.com/docs/5.3/components/card/
#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <form class="mt-1 row g-3" onsubmit={Callback::from(|event: SubmitEvent| {
                    event.prevent_default(); // Prevent the defualt form submission behavior.
                })}>
                    { for props.children.iter() }
                </form>
            </div>
        </div>
    }
}
