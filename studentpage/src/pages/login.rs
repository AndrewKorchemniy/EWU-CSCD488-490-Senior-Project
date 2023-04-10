use yew::prelude::*;

pub struct Login;
impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="col">
                // Login page ...
            </div>
        }
    }
}