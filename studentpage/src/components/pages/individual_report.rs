use yew::prelude::*;

use crate::components::atoms::collapse::{Collapse, CollapseVariant};
use crate::components::atoms::instructions::Instructions;
use crate::components::molecules::individual_form::IndividualForm;

#[function_component(IndividualReport)]
pub fn individual_report() -> Html {
    html! {
        <IndividualForm>
            <Instructions
                text="Complete all relevant fields. Refer to the tutorial (coming) for instructions. 
                <br/>
                Your time and activity account will be shared with all team members and the client."/>
            <Collapse
                label="Your Time Accounting [Public]"
                target="#collapseExample"
                variant={ CollapseVariant::Incomplete } />
            <div class="collapse show mt-0" id="collapseExample">
                <div class="card card-body border-0">
                    {"Some placeholder content for the collapse component. This panel is hidden by default but revealed when the user activates the relevant trigger."}
                </div>
            </div>

            <Collapse
                label="Your Activity Accounting [Public]"
                target="#collapseExample2"
                variant={ CollapseVariant::Invalid } />
            <div class="collapse mt-0" id="collapseExample2">
                <div class="card card-body border-0">
                    {"Some placeholder content for the collapse component. This panel is hidden by default but revealed when the user activates the relevant trigger."}
                </div>
            </div>

            <Collapse
                label="Teamate Activity Accounting [Private]"
                target="#collapseExample3"
                variant={ CollapseVariant::Complete } />
            <div class="collapse mt-0" id="collapseExample3">
                <div class="card card-body border-0">
                    {"Some placeholder content for the collapse component. This panel is hidden by default but revealed when the user activates the relevant trigger."}
                </div>
            </div>
        </IndividualForm>
    }
}
