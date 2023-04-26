use yew::prelude::*;

/// Variants of the [TextArea]
#[derive(Clone, PartialEq)]
pub enum TextAreaVariant {
    Wide,
    Split,
}

/// Properties for [TextArea]
#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub id: String,
    /// Specify the number of rows for the textarea, default value is 4
    #[prop_or("4".into())]
    pub rows: String,  
    #[prop_or_default]
    pub placeholder: String,
    //#[prop_or_default]
    //pub on_change: Callback<String>, TODO: Implement this.
    #[prop_or(TextAreaVariant::Wide)]
    pub variant: TextAreaVariant
}

/// The [TextArea] component provides a styled text area with two variants.
/// The Wide variant is a single text area that spans the entire width of the screen.
/// The Split variant spans half the width of the screen or the full screen depending on the viewport size.
#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    html! {
        {
            match props.variant {
                TextAreaVariant::Wide => html! {
                    <div class="col-12 mb-2">
                        <label for={ props.id.clone() } class="form-label"> { props.label.clone() } </label>
                        <textarea
                            class="form-control"
                            rows={ props.rows.clone() }
                            placeholder={ props.placeholder.clone() }
                            id={ props.id.clone() } />
                    </div>
                },
                TextAreaVariant::Split => html! {
                    <div class="col-12 col-xl-6 mb-2">
                        <label for={ props.id.clone() } class="form-label"> { props.label.clone() } </label>
                        <textarea
                            class="form-control"
                            rows={ props.rows.clone() }
                            placeholder={ props.placeholder.clone() }
                            id={ props.id.clone() } />
                    </div>
                }
            }
        }
    }
}