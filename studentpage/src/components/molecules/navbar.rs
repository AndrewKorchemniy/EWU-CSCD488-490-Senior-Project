use crate::components::atoms::button::Button;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar rounded ps-3 pe-2 py-1 shadow-sm my-3 font-weight-bold">
            <span>
                <Link<Route> to={Route::Home} classes="link-dark link-underline-opacity-0 link-underline-opacity-10-hover">
                    <h2> { "Status Reports" } </h2>
                </Link<Route>>
            </span>
            <span>
                <Link<Route> to={Route::Requirements}>
                    <Button variant="info" class="mx-2" label="Requirements"/>
                </Link<Route>>
                <Button variant="warning" label="Logout"/>
            </span>
        </nav>
    }
}   
