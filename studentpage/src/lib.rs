use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use components::footer::Footer;
use components::navbar::Navbar;

use pages::home::Home;
use pages::individual_report::IndividualReport;
use pages::page_not_found::PageNotFound;
use pages::requirements::Requirements;
use pages::team_report::TeamReport;

#[derive(Clone, Copy, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/requirements")]
    Requirements,
    #[at("/individual-report")]
    IndividualReport,
    #[at("/team-report")]
    TeamReport,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container-fluid mx-auto px-4">
                <div class="row">
                    <div class="col"> <Navbar /> </div>
                </div>
                <div class="row">
                    <div class="col"> <Switch<Route> render={switch} /> </div>
                </div>
                <div class="row">
                    <div class="col"> <Footer /> </div>
                </div>
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::IndividualReport => html! { <IndividualReport /> },
        Route::TeamReport => html! { <TeamReport /> },
        Route::Requirements => html! { <Requirements /> },
        Route::NotFound => html! { <PageNotFound /> },
    }
}
