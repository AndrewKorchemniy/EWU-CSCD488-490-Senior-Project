use yew::prelude::*;

/// Properties for [Collapsible]   
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The id of the element to collapse.
    pub id: AttrValue,
    /// Whether or not the element is shown by default. Defaults to false.
    #[prop_or(false)]
    pub is_open: bool,
    #[prop_or_default]
    pub children: Children,
}

/// The [Collapsible] component provides a styled wrapper for the content to collapse.
/// See <https://getbootstrap.com/docs/5.3/components/collapse/> and
/// <https://getbootstrap.com/docs/5.3/components/card/>
#[function_component(Collapsible)]
pub fn collapsible(props: &Props) -> Html {
    html! {
        <div
            class={format!("collapse mt-0 {}", if props.is_open { "show" } else { "" })}
            id={ &props.id }>
            <div class="card card-body border-0">
                { for props.children.iter() }
            </div>
        </div>
    }
}
