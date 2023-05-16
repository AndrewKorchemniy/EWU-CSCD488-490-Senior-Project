use yew::prelude::*;

/// Variants of the [TextArea]
#[derive(PartialEq)]
pub enum TextAreaVariant {
    Wide,
    Narrow,
    Split,
}

#[derive(PartialEq)]
pub enum TextAreaValidation {
    None,
    Valid,
    Invalid,
}

/// Properties for [TextArea]
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The text to display on the label.
    pub label: AttrValue,
    /// The id of the element.
    pub id: AttrValue,
    /// The number of rows to display. Defaults to 4.
    #[prop_or("4".into())]
    pub rows: AttrValue,
    /// The placeholder text to display. Defaults to an empty string.
    #[prop_or_default]
    pub placeholder: AttrValue,
    /// The value to use for the textarea. Defaults to an empty string.
    #[prop_or_default]
    pub value: AttrValue,
    /// The onchange callback.
    #[prop_or_default]
    pub onchange: Callback<Event>,
    /// The variant of the text area. Defaults to Wide.
    #[prop_or(TextAreaVariant::Wide)]
    pub variant: TextAreaVariant,
    /// The validation state of the text area. Defaults to None.
    pub validation: UseStateHandle<TextAreaValidation>,
}

/// The [TextArea] component provides a styled text area with two variants.
/// The Wide variant is a single text area that spans the entire width of the
/// screen. The Split variant scales depending on the viewport size.
#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let get_variant = |variant: &TextAreaVariant| -> String {
        match variant {
            TextAreaVariant::Wide => "col-12 mb-2",
            TextAreaVariant::Narrow => "col-12 col-xl-6 mb-2",
            TextAreaVariant::Split => "col-12 col-xl-6 mb-2",
        }
        .to_string()
    };

    let get_validation = |validation: &TextAreaValidation| -> String {
        match validation {
            TextAreaValidation::None => "",
            TextAreaValidation::Valid => "is-valid",
            TextAreaValidation::Invalid => "is-invalid",
        }
        .to_string()
    };

    let render_validation = |props: &Props| -> Html {
        html! {
            match &*props.validation {
                TextAreaValidation::None => { html! {} },
                TextAreaValidation::Valid => { html! {
                    <div class="valid-feedback ps-2" style="display: block">
                        { "Looks good!" }
                    </div>
                }},
                TextAreaValidation::Invalid => { html! {
                    <div class="invalid-feedback ps-2" style="display: block">
                        { "Please provide a valid input." }
                    </div>
                }},
            }
        }
    };

    let render_textarea = |props: &Props| -> Html {
        html! {
            <div class={ get_variant(&props.variant) }>
                <label for={ props.id.clone() } class="form-label">
                    { Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>", props.label.clone()))) }
                </label>
                <textarea
                    class={ format!("form-control {}", get_validation(&props.validation)) }
                    rows={ props.rows.clone() }
                    placeholder={ props.placeholder.clone() }
                    id={ props.id.clone() }
                    value={ props.value.clone() }
                    onchange={ &props.onchange }/>
                { render_validation(props) }
            </div>
        }
    };

    html! {
        if props.variant == TextAreaVariant::Narrow {
            <div class="col-12">
                { render_textarea(props) }
            </div>
        } else {
            { render_textarea(props) }
        }
    }
}
