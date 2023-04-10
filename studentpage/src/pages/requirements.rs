use yew::prelude::*;

pub struct Requirements;
impl Component for Requirements {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="col">
                // requirements update page ...
                <p> { "Requirements" } </p>
            </div>
        }
    }
}