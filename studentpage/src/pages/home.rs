use yew::prelude::*;

use crate::api::{api_get_sprints, SprintsResponse};
use crate::components::calendar::Calendar;
// TODO: Uncomment when the api is ready
// use crate::components::msgbox::{MsgBox, MsgBoxVariant};
use crate::components::spinner::SpinnerInset;
use crate::components::sprint::{ReportStatus, Sprint};
use crate::Route;
use yew_oauth2::context::OAuth2Context;

#[function_component(Home)]
pub fn home() -> Html {
    // Get the OAuth2Context from the agent
    let credentials = use_context::<OAuth2Context>();

    // The state of the sprints request
    let _sprints_state = use_state(|| None as Option<Result<SprintsResponse, reqwasm::Error>>);
    let sprints_state = _sprints_state.clone();
    let sprints_state_changes =
        Callback::from(move |sprints: Result<SprintsResponse, reqwasm::Error>| {
            _sprints_state.set(Some(sprints));
        });

    // Fetch the sprints if they haven't been fetched yet
    if sprints_state.is_none() {
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
                                team_report_status={ReportStatus::from(&sprint.is_team_report_submitted, sprint.due_date.clone())}
                                individual_report_status={ReportStatus::from(&sprint.is_team_report_submitted, sprint.due_date.clone())}>
                            </Sprint>
                        }
                    })}
                </Calendar>
            } else {
                // TODO: Uncomment when the api is ready
                // <MsgBox
                //     variant={ MsgBoxVariant::Danger }
                //     title="Failed to fetch sprints"
                //     text={ "Please try again later." }>
                // </MsgBox>

                // A few dummy sprints for demonstration purposes.
                <Calendar>
                    <Sprint
                        sprint_number={1}
                        due_date={"Jan 6"}
                        team_report_status={ReportStatus::Submitted}
                        individual_report_status={ReportStatus::Submitted}>
                    </Sprint>
                    <Sprint
                        sprint_number={2}
                        due_date={"Jan 13"}
                        team_report_status={ReportStatus::Missing}
                        individual_report_status={ReportStatus::Missing}>
                    </Sprint>
                    <Sprint
                        sprint_number={3}
                        due_date={"Jan 20"}
                        team_report_status={ReportStatus::Submitted}
                        individual_report_status={ReportStatus::Submitted}>
                    </Sprint>
                    <Sprint
                        sprint_number={4}
                        due_date={"Feb 3"}
                        team_report_status={ReportStatus::Submitted}
                        individual_report_status={ReportStatus::Submitted}>
                    </Sprint>
                    <Sprint
                        sprint_number={5}
                        due_date={"Feb 10"}
                        team_report_status={ReportStatus::Active(Route::TeamReport)}
                        individual_report_status={ReportStatus::Active(Route::IndividualReport)}>
                    </Sprint>
                    <Sprint
                        sprint_number={6}
                        due_date={"Feb 17"}
                        team_report_status={ReportStatus::Upcoming}
                        individual_report_status={ReportStatus::Upcoming}>
                    </Sprint>
                    <Sprint
                        sprint_number={7}
                        due_date={"Feb 24"}
                        team_report_status={ReportStatus::Upcoming}
                        individual_report_status={ReportStatus::Upcoming}>
                    </Sprint>
                    <Sprint
                        sprint_number={8}
                        due_date={"Mar 10"}
                        team_report_status={ReportStatus::Upcoming}
                        individual_report_status={ReportStatus::Upcoming}>
                    </Sprint>
                    <Sprint
                        sprint_number={9}
                        due_date={"Mar 17"}
                        team_report_status={ReportStatus::Upcoming}
                        individual_report_status={ReportStatus::Upcoming}>
                    </Sprint>
                    <Sprint
                        sprint_number={10}
                        due_date={"Mar 24"}
                        team_report_status={ReportStatus::Upcoming}
                        individual_report_status={ReportStatus::Upcoming}>
                    </Sprint>
                </Calendar>
            }
        } else {
            <SpinnerInset />
        }
    }
}
