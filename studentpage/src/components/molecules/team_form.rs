use yew::prelude::*;

use crate::components::atoms::button::Button;

/// Properties for [TeamForm]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(TeamForm)]
pub fn team_form(props: &Props) -> Html {
    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <form>
                    <div class="row g-3">
                        { for props.children.iter() }
                        <div class="row text-nowrap">
                            <div class="col">
                                <Button variant="primary" label="Submit"/>
                            </div>
                        </div>
                    </div>
                </form>
            </div>
        </div>
    }
}
