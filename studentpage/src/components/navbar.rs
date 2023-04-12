use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar py-2 px-2">
            <span style="text-decoration: none!">
                <Link<Route> to={Route::Home}>
                    <h2 style="text-decoration: none"> { "Status Reports" } </h2>
                </Link<Route>>
            </span>
            <span> 
                <Link<Route> to={Route::Requirements}>
                    <button class="btn btn-info mx-2" type="button" href="/requirements"> { "Requirements" } </button>
                </Link<Route>>
                <button class="btn btn-warning" type="button"> { "Logout" } </button>
            </span>
        </nav>
    }
}