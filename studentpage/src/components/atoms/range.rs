use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for [Range]
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The label text as HTML - wrapped in a div.
    pub label: AttrValue,
    /// The id of the range input.
    pub id: AttrValue,
    /// An optional callback to handle the input event.
    #[prop_or_default]
    pub handle_oninput: Callback<AttrValue>,
}

/// The [Range] component provides a styled range input.
/// The value of the range input is emitted to the callback as a string.
#[function_component(Range)]
pub fn range(props: &Props) -> Html {
    let handle_oninput = props.handle_oninput.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_oninput.emit(AttrValue::from(value));
    });

    html! {
        <div class="col-12">
            <div class="col-12 col-xl-6 mb-2">
                <label for={ props.id.clone() } class="form-label">
                    { Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>", props.label.clone()))) }
                </label>
                <input
                    type="range"
                    class="form-range"
                    id={ props.id.clone() }
                    min={"0"}
                    max={"100"}
                    step={"5"}
                    oninput={ oninput }/>
            </div>
        </div>
    }
}
