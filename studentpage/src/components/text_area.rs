use yew::prelude::*;

/// Variants of the [TextArea]
#[derive(PartialEq)]
pub enum TextAreaVariant {
    Wide,
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
    #[prop_or_default]
    pub is_valid: Option<UseStateHandle<TextAreaValidation>>,
}

/// The [TextArea] component provides a styled text area with two variants.
/// The Wide variant is a single text area that spans the entire width of the
/// screen. The Split variant scales depending on the viewport size.
#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let get_validation_border = |is_valid: &TextAreaValidation| -> &str {
        match is_valid {
            TextAreaValidation::None => "",
            TextAreaValidation::Valid => "is-valid",
            TextAreaValidation::Invalid => "is-invalid",
        }
    };

    let render_validation_tooltip = |is_valid: &TextAreaValidation| -> Html {
        html! {
            match is_valid {
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

    html! {
        <div class={ if props.variant == TextAreaVariant::Split {"col-12 col-xl-6"} else {"col-12"} }>
            <div class={ if props.variant == TextAreaVariant::Split {"col-12"} else {"col-12 col-xl-6"} }>
                <label for={ props.id.clone() } class="form-label">
                    { Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>", props.label.clone()))) }
                </label>
                <textarea
                    class={ format!("form-control {}",
                        if props.is_valid.is_some() {
                            get_validation_border(props.is_valid.as_ref().unwrap())
                        } else {
                            ""
                        })}
                    rows="4"
                    placeholder={ props.placeholder.clone() }
                    id={ props.id.clone() }
                    value={ props.value.clone() }
                    onchange={ &props.onchange }/>
                if props.is_valid.is_some() {
                    { render_validation_tooltip(props.is_valid.as_ref().unwrap()) }
                }
            </div>
        </div>
    }
}
