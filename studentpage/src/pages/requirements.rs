use reqwasm::Error;
use web_sys::HtmlButtonElement;
use yew::prelude::*;
use yew_oauth2::prelude::OAuth2Context;
use yew_router::prelude::*;

use crate::api::{api_get_requirements, api_post_delete_requirement};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::Card;
use crate::components::modal::Modal;
use crate::components::msgbox::{MsgBox, MsgBoxVariant};
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

    // Fetch the requirements if they haven't been fetched yet.
    if requirements_state.is_none() {
        let cloned_credentials: Option<OAuth2Context> = credentials.clone();
        let cloned_requirements_state_changes = requirements_state_changes.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let creds = cloned_credentials.unwrap();
            let token = creds.access_token().unwrap_or_default();
            let result = api_get_requirements(token).await;
            cloned_requirements_state_changes.emit(result);
        });
    }

    // The callback to delete a requirement.
    let delete_requirement = Callback::from(move |event: MouseEvent| {
        let cloned_requirements_state_changes = requirements_state_changes.clone();
        let cloned_credentials = credentials.clone();
        let target: HtmlButtonElement = event.target_unchecked_into();
        let requirement_id = target.id().parse::<i32>().unwrap_or_default();
        wasm_bindgen_futures::spawn_local(async move {
            let creds = cloned_credentials.unwrap();
            let token = creds.access_token().unwrap_or_default();
            let result = api_post_delete_requirement(requirement_id, token).await;
            cloned_requirements_state_changes.emit(result);
        });
    });

    html! {
        if requirements_state.is_some() {
            if requirements_state.as_ref().unwrap().is_ok() {
                <Card>
                    { for requirements_state.as_ref().unwrap().as_ref().unwrap().requirements.iter().map(|requirement| {
                        html! {
                            <MsgBox
                                class="mt-3 col-12 col-xl-6"
                                variant={ MsgBoxVariant::Secondary }
                                title={ format!("{}: {}",
                                    requirement.id,
                                    requirement.title.clone()) }
                                text={ requirement.description.clone() }>
                                <Button
                                    variant={ ButtonVariant::Danger }
                                    label="Delete"
                                    class="col-auto"
                                    data_bs_toggle="modal"
                                    data_bs_target={ format!("#r{}", requirement.id) } />
                                <Modal
                                    id={ format!("r{}", requirement.id) }
                                    title="Are you sure?"
                                    body="This action is irriversible."
                                    onclick={ &delete_requirement }
                                    action_id={ requirement.id.to_string() }
                                    action_label="Delete" />
                            </MsgBox>
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
