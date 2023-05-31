use reqwasm::Error;
use yew::prelude::*;
use yew_oauth2::prelude::OAuth2Context;
use yewdux::prelude::*;

use crate::api::api_post_team_report;
use crate::components::msgbox::{MsgBox, MsgBoxVariant};
use crate::components::spinner::SpinnerInset;
use crate::stores::team_store::TeamStore;
use common::models::types::TeamResponse;

#[function_component(SubmitTeamReport)]
pub fn submit_team_report() -> Html {
    // Get the OAuth2Context to get the access token.
    let credentials = use_context::<OAuth2Context>();

    // Local session store.
    let (store, _) = use_store::<TeamStore>();

    // TODO: Horrible code, needs to be better!!! Super ugly.
    let body = TeamResponse {
        understand_easy: store
            .understand_easy
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        understand_hard: store
            .understand_hard
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        approach_easy: store
            .approach_easy
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        approach_hard: store
            .approach_hard
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        solve_easy: store.solve_easy.as_deref().unwrap_or_default().to_string(),
        solve_hard: store.solve_hard.as_deref().unwrap_or_default().to_string(),
        evaluate_easy: store
            .evaluate_easy
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        evaluate_hard: store
            .evaluate_hard
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        completion_percent: store
            .completion_percent
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        pace_succeed: store
            .pace_succeed
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        client_meeting: store
            .client_meeting
            .as_deref()
            .unwrap_or_default()
            .to_string(),
        issues_comments: store
            .issues_comments
            .as_deref()
            .unwrap_or_default()
            .to_string(),
    };

    // The state of the submit request.
    let _submit_state = use_state(|| None as Option<Result<String, Error>>);
    let submit_state = _submit_state.clone();
    let submit_state_changes = Callback::from(move |response: Result<String, Error>| {
        _submit_state.set(Some(response));
    });

    if submit_state.is_none() {
        wasm_bindgen_futures::spawn_local(async move {
            let creds = credentials.unwrap();
            let token = creds.access_token().unwrap_or_default();
            let result = api_post_team_report(token, body).await;
            submit_state_changes.emit(result);
        });
    }

    html! {
        if submit_state.is_some() {
            if submit_state.as_ref().unwrap().is_ok() {
                <MsgBox
                    variant={ MsgBoxVariant::Secondary }
                    title="Response:"
                    text={ submit_state.as_ref().unwrap().as_ref().unwrap().to_owned() }>
                </MsgBox>
            } else {
                <MsgBox
                    variant={ MsgBoxVariant::Danger }
                    title="Failed to submit team report"
                    text={ "Please try again later." }>
                </MsgBox>
            }
        } else {
            <SpinnerInset />
        }
    }
}
