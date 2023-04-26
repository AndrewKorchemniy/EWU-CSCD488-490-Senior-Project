// todo: place the tab bar on top when the screen is small / on a phone
use yew::prelude::*;

#[function_component(IndividualReport)]
pub fn individual_report() -> Html {
    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <p class="card-text"> { "Individual Report" } </p>
            </div>
        </div>
    }
}
