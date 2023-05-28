use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::msgbox::{MsgBox, MsgBoxVariant};
use crate::Route;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <MsgBox
            variant={ MsgBoxVariant::Danger }
            title="Page not found!"
            text="Page does not exist.">
            <Link<Route> to={ Route::Home }>
                <Button variant={ ButtonVariant::Danger } label="Go Home" />
            </Link<Route>>
        </MsgBox>
    }
}
