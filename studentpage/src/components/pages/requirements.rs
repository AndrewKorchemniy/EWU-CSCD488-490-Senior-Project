use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::atoms::button::{Button, ButtonVariant};
use crate::components::molecules::msgbox::{MsgBox, MsgBoxVariant};
use crate::Route;

#[function_component(Requirements)]
pub fn requirements() -> Html {
    html! {
        <MsgBox
            variant={ MsgBoxVariant::Info }
            title="Requirements"
            text="This page has yet to be implemented.">
            <Link<Route> to={ Route::Home }>
                <Button variant={ ButtonVariant::Primary } label="Go Home" />
            </Link<Route>>
        </MsgBox>
    }
}
