use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::activity_accounting::ActivityAccounting;
use crate::components::button::{Button, ButtonVariant};
use crate::components::collapse::{Collapse, CollapseValidation};
use crate::components::collapsible::Collapsible;
use crate::components::form::Form;
use crate::components::instructions::Instructions;
use crate::components::modal::Modal;
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
    let time_validate = dispatch.reduce_mut_callback_with(move |store, _: MouseEvent| {
        if store.saturday_hours.is_some()
            || store.sunday_hours.is_some()
            || store.monday_hours.is_some()
            || store.tuesday_hours.is_some()
            || store.wednesday_hours.is_some()
            || store.thursday_hours.is_some()
            || store.friday_hours.is_some()
        {
            time_state_changes.emit(CollapseValidation::Complete);
        } else {
            time_state_changes.emit(CollapseValidation::Invalid);
        }
    });

    // Callback for submitting the form. Triggers client-side validation.
    let cloned_time_validate = time_validate.clone();
    let onsubmit = dispatch.reduce_mut_callback_with(move |_, event: MouseEvent| {
        event.prevent_default();
        cloned_time_validate.emit(event);
        //log!(validate_submit(store));
        // TODO: submit to server
    });

    html! {
        <Form>
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
                    label="Activity #1"
                    target="#activity1" />
                <Collapsible id="activity1" is_open={ true }>
                    { ". . ." }
                </Collapsible>
            </Collapsible>

            // <Button variant={ ButtonVariant::Danger } label="Submit" class="mt-3 col-auto ms-2" />
            <Button
                variant={ ButtonVariant::Danger }
                label="Submit"
                class="mt-3 col-auto ms-2"
                data_bs_toggle="modal"
                data_bs_target="#confirm" />
            <Modal
                id="confirm"
                title="Are you sure?"
                body="You can only submit once."
                action_label="Submit"
                action_button_type="submit"
                onclick={ onsubmit } />
        </Form>
    }
}
