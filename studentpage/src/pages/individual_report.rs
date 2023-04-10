use yew::prelude::*;

pub struct IndividualReport;
impl Component for IndividualReport {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="col">
                // individual reports page ...
            </div>
        }
    }
}