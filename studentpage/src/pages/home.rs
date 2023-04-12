use yew::prelude::*;

use crate::components::calendar::Calendar;
use crate::components::sprint::Sprint;
use crate::components::sprint::ReportStatus;
use crate::Route;

#[function_component]
pub fn Home() -> Html {
    html! {
        <Calendar>
            // a few dummy calendar entries that pass in fake data into the calendar component for demonstration purposes
            <Sprint
                sprint={1}
                due_date={"Jan 6"}
                team_report_status={ReportStatus::Submitted}
                individual_report_status={ReportStatus::Submitted}>
            </Sprint>
            <Sprint
                sprint={2}
                due_date={"Jan 13"}
                team_report_status={ReportStatus::Missing}
                individual_report_status={ReportStatus::Missing}>
            </Sprint>
            <Sprint
                sprint={3}
                due_date={"Jan 20"}
                team_report_status={ReportStatus::Submitted}
                individual_report_status={ReportStatus::Submitted}>
            </Sprint>
            <Sprint
                sprint={4}
                due_date={"Feb 3"}
                team_report_status={ReportStatus::Submitted}
                individual_report_status={ReportStatus::Submitted}>
            </Sprint>
            <Sprint
                sprint={5}
                due_date={"Feb 10"}
                team_report_status={ReportStatus::Active(Route::TeamReport)}
                individual_report_status={ReportStatus::Active(Route::IndividualReport)}>
            </Sprint>
            <Sprint
                sprint={6}
                due_date={"Feb 17"}
                team_report_status={ReportStatus::Upcoming}
                individual_report_status={ReportStatus::Upcoming}>
            </Sprint>
            <Sprint
                sprint={7}
                due_date={"Feb 24"}
                team_report_status={ReportStatus::Upcoming}
                individual_report_status={ReportStatus::Upcoming}>
            </Sprint>
            <Sprint
                sprint={8}
                due_date={"Mar 10"}
                team_report_status={ReportStatus::Upcoming}
                individual_report_status={ReportStatus::Upcoming}>
            </Sprint>
            <Sprint
                sprint={9}
                due_date={"Mar 17"}
                team_report_status={ReportStatus::Upcoming}
                individual_report_status={ReportStatus::Upcoming}>
            </Sprint>
            <Sprint
                sprint={10}
                due_date={"Mar 24"}
                team_report_status={ReportStatus::Upcoming}
                individual_report_status={ReportStatus::Upcoming}>
            </Sprint>
        </Calendar>
    }
}
