use reqwasm::Error;
use yew::prelude::*;

use crate::api::api_get_sprints;
use crate::components::calendar::Calendar;
use crate::components::msgbox::{MsgBox, MsgBoxVariant};
use crate::components::spinner::SpinnerInset;
use crate::components::sprint::{ReportStatus, Sprint};
use crate::Route;
use common::models::types::SprintsResponse;
use yew_oauth2::context::OAuth2Context;

#[function_component(Home)]
pub fn home() -> Html {
    // Get the OAuth2Context to get the access token.
    let credentials = use_context::<OAuth2Context>();

    // The state of the sprints request.
    let _sprints_state = use_state(|| None as Option<Result<SprintsResponse, Error>>);
    let sprints_state = _sprints_state.clone();
    let sprints_state_changes =
        Callback::from(move |sprints: Result<SprintsResponse, reqwasm::Error>| {
            _sprints_state.set(Some(sprints));
        });

    // The state for if the call was sent.
    let called_sprints_state = use_state(|| false);

    // Fetch the sprints if they haven't been fetched yet.
    if !*called_sprints_state {
        called_sprints_state.set(true);
        wasm_bindgen_futures::spawn_local(async move {
            let creds = credentials.unwrap();
            let token = creds.access_token().unwrap_or_default();
            let result = api_get_sprints(token).await;
            sprints_state_changes.emit(result);
        });
    }

    html! {
        if sprints_state.is_some() {
            if sprints_state.as_ref().unwrap().is_ok() {
                <Calendar>
                    // Render the sprints from sprint_state.
                   { for sprints_state.as_ref().unwrap().as_ref().unwrap().sprints.iter().map(|sprint| {
                        html! {
                            <Sprint
                                sprint_number={sprint.id}
                                due_date={sprint.due_date.format("%b %e").to_string()}
                                team_report_status={ReportStatus::from(
                                    &sprint.is_team_report_submitted,
                                    sprint.due_date,
                                    Route::TeamReport
                                )}
                                individual_report_status={ReportStatus::from(
                                    &sprint.is_team_report_submitted,
                                    sprint.due_date,
                                    Route::IndividualReport
                                )}>
                            </Sprint>
                        }
                    })}
                </Calendar>
            } else {
                <MsgBox
                    variant={ MsgBoxVariant::Danger }
                    title="Failed to fetch sprints"
                    text={ "Please try again later." }>
                </MsgBox>
            }
        } else {
            <SpinnerInset />
        }
    }
}
