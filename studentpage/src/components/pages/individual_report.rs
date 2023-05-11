use yew::prelude::*;

use crate::components::atoms::collapse::{Collapse, CollapseVariant};
use crate::components::atoms::collapsible::Collapsible;
use crate::components::atoms::instructions::Instructions;
use crate::components::molecules::activity_accounting::ActivityAccounting;
use crate::components::molecules::individual_form::IndividualForm;
use crate::components::molecules::time_accounting::TimeAccounting;

#[function_component(IndividualReport)]
pub fn individual_report() -> Html {
    html! {
        <IndividualForm>
            <Instructions
                text="Complete all relevant fields. Refer to the tutorial (coming) for instructions. <br/> Your time and activity account will be shared with all team members and the client."/>

            <Collapse
                label="Your Time Accounting [Public]"
                variant={ CollapseVariant::Incomplete }
                target="#time-accounting" />
            <Collapsible id="time-accounting" is_open={ true }>
                { "What was your effort on the project during this sprint? Round to the nearest 15 minutes." }
                <TimeAccounting />
            </Collapsible>

            <Collapse
                label="Your Activity Accounting [Public]"
                variant={ CollapseVariant::Invalid }
                target="#activity-accounting" />
            <Collapsible id="activity-accounting">
                { "Enter any new activities that you started during this sprint. They are assigned to you until at least the next sprint. There is no significance to the order, and activity codes may not be sequential. Choose a short, meaningful title that is a convenient, human-firnedly reference. The description should be a concise summary of on thing that is to be done. Break larger tasks into multiple activities, but do not get carried away. In subsequent sprints, you will need to account for the status of each until they are closed. Estimate how many sprints you expect the activity to take. Finally, associate this activity with any requirements that it addresses. Is is possible to have an activity without an explicit requirement (e.g., initially setting up the development server), but is unlikely once the project is going. Everything you are doing needs to be attributed to a reason from a source, which is primarily the requirements." }

                <ActivityAccounting />
            </Collapsible>

            <Collapse
                label="Teamate Activity Accounting [Private]"
                variant={ CollapseVariant::Complete }
                target="#team-acitivty-accounting" />
            <Collapsible id="team-acitivty-accounting">
                {"Some placeholder content for the collapse component. This panel is hidden by default but revealed when the user activates the relevant trigger."}

                <Collapse
                    label="Testing"
                    target="#testing2" />
                <Collapsible id="testing2" is_open={ true }>
                    {"noice"}
                </Collapsible>
            </Collapsible>
        </IndividualForm>
    }
}
