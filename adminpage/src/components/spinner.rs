use yew::prelude::*;

/// The [Spinner] component provides a bootstrap spinner.
/// This variant is meant to be used to fill the entire screen when nothing renders.
/// See <https://getbootstrap.com/docs/5.3/components/spinners/>
#[function_component(Spinner)]
pub fn spinner() -> Html {
    html! {
        <div class="spinner-wrapper text-primary">
            <div class="spinner-border" role="status" style="width: 6rem; height: 6rem;">
                <span class="visually-hidden"> { "Loading..." } </span>
            </div>
        </div>
    }
}

/// The [SpinnerInset] component provides a bootstrap spinner.
/// The inset variant is meant to be used on a loaded page.
/// See <https://getbootstrap.com/docs/5.3/components/spinners/>
#[function_component(SpinnerInset)]
pub fn spinner_inset() -> Html {
    html! {
        <div class="text-primary ms-3">
            <div class="spinner-border" role="status">
                <span class="visually-hidden"> { "Loading..." } </span>
            </div>
        </div>
    }
}
