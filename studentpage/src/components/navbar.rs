use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar rounded ps-3 pe-2 py-1 shadow-sm my-3 font-weight-bold">
            <span>
                <Link<Route> to={Route::Home} classes="link-dark link-underline-opacity-0 link-underline-opacity-10-hover">
                    <h2> { "Status Reports" } </h2>
                </Link<Route>>
            </span>
            <span>
                <Link<Route> to={Route::Requirements}>
                    <button class="btn btn-info mx-2 shadow"> { "Requirements" } </button>
                </Link<Route>>
                <button class="btn btn-outline btn-warning shadow"> { "Logout" } </button>
            </span>
        </nav>
    }
}