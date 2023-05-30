use yew::prelude::*;

/// Properties for [Modal]   
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Classes to apply to the button.
    #[prop_or_default]
    pub class: Classes,
    /// The id of the modal.
    pub id: AttrValue,
    /// The title of the modal.
    pub title: AttrValue,
    /// The body of the modal.
    pub body: AttrValue,
    /// The label for the action button.
    pub action_label: AttrValue,
    /// The type of the action button.
    #[prop_or_default]
    pub action_button_type: AttrValue,
    /// The onclick callback for the action.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// The [Modal] component provides styled bootstrap modal (a dialog popup).
/// See https://getbootstrap.com/docs/5.3/components/modal/
#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    html! {
        <div class="modal fade" id={ props.id.clone() } tabindex="-1" aria-labelledby={ props.id.clone() } aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="modal-title fs-5" id="exampleModalLabel">{ props.title.clone() }</h1>
                    </div>
                    <div class="modal-body">
                        { props.body.clone() }
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" aria-label="Close">{ "Close" }</button>
                        <button
                            type="button"
                            class="btn btn-danger"
                            data-bs-dismiss="modal"
                            aria-label="Submit"
                            action_button_type={ props.action_button_type.clone() }
                            onclick={ &props.onclick }>
                            { props.action_label.clone() }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
