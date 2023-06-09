use yew::prelude::*;

/// Properties for [Card]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

/// The [Card] component provides a styled boostrap card wrapper.
/// See <https://getbootstrap.com/docs/5.3/components/card/>
#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <div class="row g-3">
                    <div class="mt-0 col-12">
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        </div>
    }
}
