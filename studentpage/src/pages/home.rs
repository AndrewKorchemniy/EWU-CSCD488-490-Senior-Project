use yew::prelude::*;

use crate::components::calendar::Calendar;
use crate::components::sprint::{ReportStatus, Sprint};
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Calendar>
            // a few dummy sprints for demonstration purposes
            // communicate with the backend to get the actual sprint data
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
}
