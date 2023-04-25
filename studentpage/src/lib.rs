use yew::prelude::*;
use yew_router::prelude::*;

mod components;

use components::molecules::footer::Footer;
use components::molecules::navbar::Navbar;

use components::pages::home::Home;
use components::pages::individual_report::IndividualReport;
use components::pages::page_not_found::PageNotFound;
use components::pages::requirements::Requirements;
use components::pages::team_report::TeamReport;

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
