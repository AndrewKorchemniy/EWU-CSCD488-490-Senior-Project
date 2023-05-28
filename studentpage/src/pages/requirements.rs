use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::msgbox::{MsgBox, MsgBoxVariant};
use crate::Route;

#[function_component(Requirements)]
pub fn requirements() -> Html {
    html! {
        <MsgBox
            variant={ MsgBoxVariant::Warning }
            title="Requirements"
            text="This page has yet to be implemented.">
            <Link<Route> to={ Route::Home }>
                <Button variant={ ButtonVariant::Warning } label="Go Home" />
            </Link<Route>>
        </MsgBox>
    }
}
