use gloo_utils::window;
use reqwasm::Error;
use yew::prelude::*;
use yew_oauth2::prelude::OAuth2Context;
use yew_router::prelude::*;

use crate::api::api_get_requirements;
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::Card;
use crate::components::msgbox::{MsgBox, MsgBoxVariant};
use crate::components::requirement::Requirement;
use crate::components::spinner::SpinnerInset;
use crate::Route;

use common::models::types::RequirementsResponse;

#[function_component(Requirements)]
pub fn requirements() -> Html {
    // Get the OAuth2Context to get the access token.
    let credentials = use_context::<OAuth2Context>();

    // The state of the requirements request.
    let _requirements_state = use_state(|| None as Option<Result<RequirementsResponse, Error>>);
    let requirements_state = _requirements_state.clone();
    let requirements_state_changes = Callback::from(
        move |requirements: Result<RequirementsResponse, reqwasm::Error>| {
            _requirements_state.set(Some(requirements));
        },
    );

    // The state for if the call was sent.
    let called_requirements_state = use_state(|| false);

    // Fetch the requirements if they haven't been fetched yet.
    if !*called_requirements_state {
        called_requirements_state.set(true);
        let cloned_requirements_state_changes = requirements_state_changes.clone();
        let creds = credentials.clone().unwrap();
        wasm_bindgen_futures::spawn_local(async move {
            let token = creds.access_token().unwrap_or_default();
            let result = api_get_requirements(token).await;
            cloned_requirements_state_changes.emit(result);
        });
    }

    // The state for if a delete call was sent.
    let called_delete_state = use_state(|| false);
    let cloned_called_delete_state = called_delete_state.clone();

    // Reload the page if delete was called.
    if *called_delete_state {
        let document = window();
        _ = document.location().reload();
    }

    html! {
        if requirements_state.is_some() {
            if requirements_state.as_ref().unwrap().is_ok() {
                <Card>
                    { for requirements_state.as_ref().unwrap().as_ref().unwrap().requirements.iter().map(|requirement| {
                        html! {
                            <Requirement
                                id={ requirement.id }
                                title={ requirement.title.clone() }
                                description={ requirement.description.clone() }
                                deleted={ cloned_called_delete_state.clone() } />
                        }
                    })}
                    <Link<Route> to={ Route::Home }>
                        <Button
                            variant={ ButtonVariant::Warning }
                            label="Exit"
                            class="col-auto  mt-3" />
                    </Link<Route>>
                    <Button
                        variant={ ButtonVariant::Success }
                        label="Create Requirement"
                        class="col-auto ms-3 mt-3" />

                </Card>
            } else {
                <MsgBox
                    variant={ MsgBoxVariant::Danger }
                    title="Failed to fetch requirements"
                    text="Please try again later.">
                    <Link<Route> to={ Route::Home }>
                        <Button variant={ ButtonVariant::Danger } label="Go Home" />
                    </Link<Route>>
                </MsgBox>
            }
        } else {
            <SpinnerInset />
        }
    }
}
