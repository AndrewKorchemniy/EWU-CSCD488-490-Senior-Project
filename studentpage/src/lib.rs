use gloo::console::log;
use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*;
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

#[function_component(AppMain)]
pub fn app_main() -> Html {
    let agent = use_auth_agent().expect("Requires OAuth2Context");

    // TODO: Add OAuth2
    // let login = {
    //     let agent = agent.clone();
    //     Callback::from(move |_: MouseEvent| {
    //         if let Err(err) = agent.start_login() {
    //             log!(format!("Failed to start login: {err}"));
    //         }
    //     })
    // };

    let logout = Callback::from(move |_: MouseEvent| {
        if let Err(err) = agent.start_login() {
            log!(format!("Failed to logout: {err}"));
        }
    });

    html! {
        <BrowserRouter>
            <div class="container-fluid mx-auto px-4" style="min-width: 480px;">
                <div class="row">
                    <div class="col"> <Navbar logout={logout}/> </div>
                </div>
                <div class="row">
                    <div class="col"> <Switch<Route> render={switch} /> </div>
                </div>
                <div class="row">
                    <div class="col"> <Footer /> </div>
                </div>
            </div>
        </BrowserRouter>
        // TODO: Add OAuth2
        // <OAuth2 {config}>
        //     <Authenticated>
        //         <BrowserRouter>
        //             <div class="container-fluid mx-auto px-4" style="min-width: 480px;">
        //                 <div class="row">
        //                     <div class="col"> <Navbar logout={logout}/> </div>
        //                 </div>
        //                 <div class="row">
        //                     <div class="col"> <Switch<Route> render={switch} /> </div>
        //                 </div>
        //                 <div class="row">
        //                     <div class="col"> <Footer /> </div>
        //                 </div>
        //             </div>
        //         </BrowserRouter>
        //     </Authenticated>
        //     <NotAuthenticated>
        //         <p>
        //             { "You need to log in" }
        //             <button onclick={login}> { "Log in" } </button>
        //         </p>
        //     </NotAuthenticated>
        // </OAuth2>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let config = Config {
        client_id: "client_id".into(),
        auth_url: "auth_url".into(),
        token_url: "token_url".into(),
    };

    html! {
        <OAuth2 config={config}>
            <AppMain />
        </OAuth2>
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
