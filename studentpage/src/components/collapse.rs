use yew::prelude::*;

/// Variants for [Collapse]
#[allow(dead_code)] // TODO: remove if some variants never get used
#[derive(PartialEq, Copy, Clone)]
pub enum CollapseValidation {
    Incomplete,
    Complete,
    Invalid,
}

/// Properties for [Collapse]   
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The text to display on the button.
    pub label: AttrValue,
    /// The id of the element to collapse.
    pub target: AttrValue,
    /// The variant of the button.
    #[prop_or(CollapseValidation::Incomplete)]
    pub variant: CollapseValidation,
    /// The function to call when the button is clicked.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// The [Collapse] component provides a styled collapse button.
/// Collapses the element with the given id when clicked.
#[function_component(Collapse)]
pub fn collapse(props: &Props) -> Html {
    let get_btn_variant = |variant: &CollapseValidation| match variant {
        CollapseValidation::Incomplete => "btn-light",
        CollapseValidation::Complete => "btn-success",
        CollapseValidation::Invalid => "btn-danger",
    };

    let get_icon_variant = |variant: &CollapseValidation| match variant {
        CollapseValidation::Incomplete => html! { <i class="fas fa-minus fa-xl me-2"></i>},
        CollapseValidation::Complete => html! { <i class="fas fa-check fa-xl me-2"></i>},
        CollapseValidation::Invalid => html! { <i class="fas fa-xmark fa-xl me-2"></i>},
    };

    html! {
        <div class="col-12 mt-3">
            <button
                type="button"
                onclick={ &props.onclick }
                class={ format!("btn shadow text-start col-auto {}", get_btn_variant(&props.variant)) }
                data-bs-toggle="collapse"
                data-bs-target={ props.target.clone() }
                style="background-image: linear-gradient(135deg, #FFFFFF20, #00000020)">
            { get_icon_variant(&props.variant) }
            { &*props.label }
            </button>
        </div>
    }
}
