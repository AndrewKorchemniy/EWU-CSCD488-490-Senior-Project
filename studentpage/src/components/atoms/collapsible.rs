use yew::prelude::*;

/// Properties for [Collapsible]   
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The id of the element to collapse.
    pub id: AttrValue,
    /// Whether or not the element is shown by default. Defaults to false.
    #[prop_or(false)]
    pub show: bool,
    #[prop_or_default]
    pub children: Children,
}

/// The [Collapsible] component provides a styled wrapper for the content to collapse.
#[function_component(Collapsible)]
pub fn collapsible(props: &Props) -> Html {
    let get_show = |show: &bool| -> &str {
        match show {
            true => "show",
            false => "",
        }
    };

    html! {
        <div 
            class={format!("collapse mt-0 {}", get_show(&props.show))} 
            id={ &props.id }>
            <div class="card card-body border-0">
                { for props.children.iter() }
            </div>
        </div>
    }
}
