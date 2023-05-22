use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::molecules::footer::Footer;
use components::molecules::navbar::Navbar;
use components::pages::home::Home;
use components::pages::page_not_found::PageNotFound;

mod stores;

#[derive(Clone, Copy, Routable, PartialEq)]
pub enum Route {
    #[at("/adminpage")]
    Home,
    #[not_found]
    #[at("/adminpage/404")]
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
        Route::NotFound => html! { <PageNotFound /> },
    }
}
