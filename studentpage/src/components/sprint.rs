use crate::Route;
use chrono::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// The status of a report entry for [Sprint]
#[derive(PartialEq)]
pub enum ReportStatus {
    Submitted,
    Missing,
    Upcoming,
    Active(Route),
}

impl ReportStatus {
    /// Returns the appropriate ReportStatus based on the given parameters.
    pub fn from(is_completed: &bool, due_date: NaiveDate, route: Route) -> Self {
        match is_completed {
            true => ReportStatus::Submitted,
            false => {
                let now = Local::now().naive_local().date();
                if now > due_date {
                    ReportStatus::Missing
                } else if now == due_date {
                    ReportStatus::Active(route)
                } else {
                    ReportStatus::Upcoming
                }
            }
        }
    }
}

/// Properties for [Sprint]
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The sprint number.
    pub sprint_number: u8,
    /// The due date of the sprint.
    pub due_date: AttrValue,
    /// The status of the team report.
    pub team_report_status: ReportStatus,
    /// The status of the individual report.
    pub individual_report_status: ReportStatus,
}

/// The [Sprint] component provides a row for the Calendar table.
/// It takes in the sprint number, due date, and status of the team and individual reports.
/// Each row within the calendar represents a sprint.
#[function_component(Sprint)]
pub fn sprint(props: &Props) -> Html {
    // Renders the appropriate icon given the report status.
    let get_status = |status: &ReportStatus| -> Html {
        match status {
            ReportStatus::Submitted => html! {
                <td class="text-center" style="color: limegreen">
                    <i class="fas fa-check fa-xl"></i>
                </td>
            },
            ReportStatus::Missing => html! {
                <td class="text-center" style="color: tomato">
                    <i class="fas fa-xmark fa-xl"></i>
                </td>
            },
            ReportStatus::Upcoming => html! {
                <td class="text-center" style="color: darkgray">
                    <i class="fas fa-minus fa-xl"></i>
                </td>
            },
            ReportStatus::Active(route) => html! {
                // If the report is active, render a link to the report page.
                <td class="text-center" style="color: dodgerblue">
                    <Link<Route> to={*route}>
                        <i class="fas fa-arrow-right fa-xl"></i>
                    </Link<Route>>
                </td>
            },
        }
    };

    html! {
        <tr>
            <td class="text-center text-nowrap">{ &props.sprint_number }</td>
            <td class="text-left text-nowrap">{ &props.due_date }</td>
            { get_status(&props.individual_report_status) }
            { get_status(&props.team_report_status) }
            <td></td>
        </tr>
    }
}
