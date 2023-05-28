use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::activity_accounting::ActivityAccounting;
use crate::components::collapse::{Collapse, CollapseValidation};
use crate::components::collapsible::Collapsible;
use crate::components::individual_form::IndividualForm;
use crate::components::instructions::Instructions;
use crate::components::time_accounting::TimeAccounting;

use crate::stores::individual_store::IndividualStore;

#[function_component(IndividualReport)]
pub fn individual_report() -> Html {
    // Local session store.
    let (store, dispatch) = use_store::<IndividualStore>();

    // Validation state for each primary collapse section.
    let _time_state = use_state(|| CollapseValidation::Incomplete);
    let time_state = _time_state.clone();
    let time_state_changes = Callback::from(move |value: CollapseValidation| {
        _time_state.set(value);
    });
    let _activity_state = use_state(|| CollapseValidation::Incomplete);
    let activity_state = _activity_state.clone();
    // let activity_state_changes = Callback::from(move |value: CollapseValidation| {
    //     _activity_state.set(value);
    // });
    let _team_activity_state = use_state(|| CollapseValidation::Incomplete);
    let team_activity_state = _team_activity_state.clone();
    // let team_activity_state_changes = Callback::from(move |value: CollapseValidation| {
    //     _team_activity_state.set(value);
    // });

    // Time accounting validation.
    let cloned_store = store.clone();
    let time_validate = Callback::from(move |_: MouseEvent| {
        if cloned_store.saturday_hours.is_some()
            || cloned_store.sunday_hours.is_some()
            || cloned_store.monday_hours.is_some()
            || cloned_store.tuesday_hours.is_some()
            || cloned_store.wednesday_hours.is_some()
            || cloned_store.thursday_hours.is_some()
            || cloned_store.friday_hours.is_some()
        {
            time_state_changes.emit(CollapseValidation::Complete);
        } else {
            time_state_changes.emit(CollapseValidation::Invalid);
        }
    });

    html! {
        <IndividualForm>
            <Instructions
                text="Complete all relevant fields. Refer to the tutorial (coming) for instructions. <br/> Your time and activity account will be shared with all team members and the client."/>

            <Collapse
                label="Your Time Accounting [Public]"
                variant={ *time_state }
                onclick={ time_validate }
                target="#time-accounting" />
            <Collapsible id="time-accounting" is_open={ true }>
                { "What was your effort on the project during this sprint? Round to the nearest 15 minutes." }
                <TimeAccounting store={ store } dispatch={ dispatch } />
            </Collapsible>

            <Collapse
                label="Your Activity Accounting [Public]"
                variant={ *activity_state }
                target="#activity-accounting" />
            <Collapsible id="activity-accounting">
                { "Enter any new activities that you started during this sprint. They are assigned to you until at least the next sprint. There is no significance to the order, and activity codes may not be sequential. Choose a short, meaningful title that is a convenient, human-firnedly reference. The description should be a concise summary of on thing that is to be done. Break larger tasks into multiple activities, but do not get carried away. In subsequent sprints, you will need to account for the status of each until they are closed. Estimate how many sprints you expect the activity to take. Finally, associate this activity with any requirements that it addresses. Is is possible to have an activity without an explicit requirement (e.g., initially setting up the development server), but is unlikely once the project is going. Everything you are doing needs to be attributed to a reason from a source, which is primarily the requirements." }

                <ActivityAccounting />
            </Collapsible>

            <Collapse
                label="Teamate Activity Accounting [Private]"
                variant={ *team_activity_state }
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
