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

mod stores;

#[derive(Clone, Copy, Routable, PartialEq)]
pub enum Route {
    #[at("/studentpage")]
    Home,
    #[at("/studentpage/requirements")]
    Requirements,
    #[at("/studentpage/individual-report")]
    IndividualReport,
    #[at("/studentpage/team-report")]
    TeamReport,
    #[not_found]
    #[at("/studentpage/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container-fluid mx-auto px-4" style="min-width: 480px;">
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
