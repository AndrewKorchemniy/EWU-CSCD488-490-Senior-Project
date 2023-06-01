use yew::prelude::*;
use yew_oauth2::prelude::OAuth2Context;

use crate::{components::{msgbox::{MsgBox, MsgBoxVariant}, button::{Button, ButtonVariant}, modal::Modal}, api::api_post_delete_requirement};

/// Properties for [Requirement]
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The requirement id.
    pub id: i32,
    /// The requirement title.
    pub title: AttrValue,
    /// The requirement description.
    pub description: AttrValue,
    /// The UseStateHandle for if deletion was called.
    pub deleted: UseStateHandle<bool>,
}


/// The [Requirement] component provides a styled MsgBox
/// for displaying requirements.
/// When the requirement is deleted, is sets the deleted state to true
/// within the parent component.
#[function_component(Requirement)]
pub fn reuirement(props: &Props) -> Html {
    // Get the OAuth2Context to get the access token.
    let credentials = use_context::<OAuth2Context>();

    // The state for the call to delete a requirement.
    let called_delete_state = use_state(|| false);

    // The delete requirement callback.
    let id = props.id;
    let cloned_called_delete_state = called_delete_state.clone();
    let delete_requirement = Callback::from(move |_: MouseEvent| {
        let cloned_credentials = credentials.clone();
        let cloned_called_delete_state = cloned_called_delete_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let creds = cloned_credentials.unwrap();
            let token = creds.access_token().unwrap_or_default();
            _ = api_post_delete_requirement(id, token).await;
            // TODO: Use result to display an error.
            cloned_called_delete_state.set(true);
        });
    });

    // Inform parent component that a requirement was deleted.
    if *called_delete_state {
        props.deleted.set(true);
    }

    html! {
        <MsgBox
            class="mt-3 col-12 col-xl-6"
            variant={ MsgBoxVariant::Secondary }
            title={ format!("{}: {}",
                props.id,
                props.title.clone()) }
            text={ props.description.clone() }>
            <Button
                variant={ ButtonVariant::Danger }
                label="Delete"
                class="col-auto"
                data_bs_toggle="modal"
                data_bs_target={ format!("#r{}", props.id) } />
            <Modal
                id={ format!("r{}", props.id) }
                title="Are you sure?"
                body="This action is irriversible."
                onclick={ &delete_requirement }
                action_id={ props.id.to_string() }
                action_label="Delete" />
        </MsgBox>
    }
}
