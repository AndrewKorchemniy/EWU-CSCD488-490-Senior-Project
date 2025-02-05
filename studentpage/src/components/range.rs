use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for [Range]
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The label text as HTML - wrapped in a div.
    pub label: AttrValue,
    /// The id of the range input.
    pub id: AttrValue,
    /// The onchange callback.
    #[prop_or_default]
    /// The initial value.
    #[prop_or_default]
    pub initial_value: AttrValue,
    pub onchange: Callback<Event>,
    /// An optional callback to handle the input event.
    #[prop_or_default]
    pub handle_oninput: Callback<AttrValue>,
}

/// The [Range] component provides a styled range input.
/// The value of the range input is emitted to the callback as a string.
#[function_component(Range)]
pub fn range(props: &Props) -> Html {
    // Local state for the range input since Yew's rendering engine handles HTML values.
    // This would normally be handled by the browser. However, it also allows for
    // a default value to be set (-1) in the event it is not set.
    let _range_state = use_state(|| props.initial_value.clone());
    let range_state = _range_state.clone();
    let range_changed = Callback::from(move |value: AttrValue| {
        _range_state.set(value);
    });

    let handle_oninput = props.handle_oninput.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let value = event.target_unchecked_into::<HtmlInputElement>().value();
        range_changed.emit(AttrValue::from(value.clone()));
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
                    min="0"
                    max="100"
                    value= { &*range_state } // Must use state here to use a default value.
                    step="5"
                    draggable={ "false" }
                    oninput={ oninput }
                    onchange={ &props.onchange }/>
            </div>
        </div>
    }
}
