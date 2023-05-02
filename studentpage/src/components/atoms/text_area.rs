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
    pub label: String,
    pub id: String,
    #[prop_or(true)]
    pub required: bool,
    #[prop_or("4".into())]
    pub rows: String,
    #[prop_or_default]
    pub placeholder: String,
    //#[prop_or_default]
    //pub on_change: Callback<String>, TODO: Implement this.
    #[prop_or(TextAreaVariant::Wide)]
    pub variant: TextAreaVariant,
}

/// The [TextArea] component provides a styled text area with two variants.
/// The Wide variant is a single text area that spans the entire width of the screen.
/// The Split variant spans half the width of the screen or the full screen depending on the viewport size.
#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let get_classes = |variant: &TextAreaVariant| -> String {
        match variant {
            TextAreaVariant::Wide => "col-12 mb-2",
            TextAreaVariant::Split => "col-12 col-xl-6 mb-2",
        }
        .into()
    };

    html! {
        <div class={ get_classes(&props.variant) }>
            <label for={ props.id.clone() } class="form-label"> 
                { Html::from_html_unchecked(AttrValue::from(props.label.clone())) } 
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
