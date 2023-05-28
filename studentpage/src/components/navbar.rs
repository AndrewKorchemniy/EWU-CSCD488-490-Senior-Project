use crate::components::button::{Button, ButtonVariant};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Properties for [NavBar].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The onclick callback for logging out.
    pub logout: Callback<MouseEvent>,
}

/// The [Navbar] component provides a styled navbar for the application.
#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    // The requirements button is conditionally rendered based on the current route.
    let route: Option<Route> = use_route::<Route>();
    let is_route_home = || matches!(route.unwrap(), Route::Home);

    html! {
        <nav class="navbar rounded ps-3 pe-2 py-1 shadow my-3 font-weight-bold">
            <span class="navbar-link">
                <Link<Route> to={ Route::Home } classes="link-dark link-underline-opacity-0
                    link-underline-opacity-10-hover">
                    <h2 class="mb-1"> { "Status Reports" } </h2>
                </Link<Route>>
            </span>
            <span class="my-1">
                if is_route_home() {
                    <Link<Route> to={ Route::Requirements }>
                        <Button
                            variant={ ButtonVariant::Primary }
                            class="me-2"
                            label="Requirements"
                            onclick={&props.logout} />
                    </Link<Route>>
                }
                <Button variant={ ButtonVariant::Warning } label="Logout" />
            </span>
        </nav>
    }
}
