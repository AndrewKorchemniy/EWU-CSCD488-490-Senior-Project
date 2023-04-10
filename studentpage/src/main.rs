use yew_router::prelude::*;
use yew::prelude::*;

mod components;
use components::navbar::Navbar;
use components::footer::Footer;
mod pages;
use pages::home::Home;
use pages::individual_report::IndividualReport;
use pages::team_report::TeamReport;
use pages::requirements::Requirements;
use pages::page_not_found::PageNotFound;

#[derive(Clone, Routable, PartialEq)]
enum Route {
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
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container-fluid" style="width: 95%">
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

fn main() {
    yew::Renderer::<App>::new().render();
}