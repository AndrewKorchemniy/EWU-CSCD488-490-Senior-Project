use yew::prelude::*;

/// Variants of the [TextArea]
#[derive(PartialEq)]
pub enum TextAreaVariant {
    Wide,
    Split,
}

/// Properties for [TextArea]
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The text to display on the label.
    pub label: AttrValue,
    /// The id of the element.
    pub id: AttrValue,
    /// Whether or not the element is required. Defaults to true.
    #[prop_or(true)]
    pub required: bool,
    /// The number of rows to display. Defaults to 4.
    #[prop_or("4".into())]
    pub rows: AttrValue,
    /// The placeholder text to display. Defaults to an empty string.
    #[prop_or_default]
    pub placeholder: AttrValue,
    /// The variant of the text area. Defaults to Wide.
    #[prop_or(TextAreaVariant::Wide)]
    pub variant: TextAreaVariant,
}

/// The [TextArea] component provides a styled text area with two variants.
/// The Wide variant is a single text area that spans the entire width of the 
/// screen. The Split variant scales depending on the viewport size.
#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let get_variant = |variant: &TextAreaVariant| -> String {
        match variant {
            TextAreaVariant::Wide => "col-12 mb-2",
            TextAreaVariant::Split => "col-12 col-xl-6 mb-2",
        }
        .to_string()
    };

    html! {
        <div class={ get_variant(&props.variant) }>
            <label for={ props.id.clone() } class="form-label">
                { Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>", props.label.clone()))) }
            </label>
            <textarea
                class="form-control"
                rows={ props.rows.clone() }
                placeholder={ props.placeholder.clone() }
                id={ props.id.clone() }
                required={ props.required }/>
        </div>
    }
}
