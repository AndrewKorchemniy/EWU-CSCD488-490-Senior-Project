use crate::Route;
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

/// Properties for [Sprint]
#[derive(Properties, PartialEq)]
pub struct Props {
    pub sprint_number: u8,
    pub due_date: String,
    pub team_report_status: ReportStatus,
    pub individual_report_status: ReportStatus,
}

/// The [Sprint] component provides a row for the table within the [Calendar] component.
/// It takes in the sprint number, due date, and status of the team and individual reports.
/// Each row within the calendar represents a sprint.
#[function_component(Sprint)]
pub fn sprint(props: &Props) -> Html {
    // render the corresponding element for the status of the report
    fn get_status(status: &ReportStatus) -> Html {
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
                // if the report is active, render a link to the report page
                <td class="text-center" style="color: dodgerblue">
                    <Link<Route> to={*route}>
                        <i class="fas fa-arrow-right fa-xl"></i>
                    </Link<Route>>
                </td>
            },
        }
    }

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
