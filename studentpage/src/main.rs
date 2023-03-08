mod IndividualReport;

use yew::prelude::*;

pub struct Model {
    pub value: i64
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| Model {
        value: 100
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value - 1
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ state.value }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}