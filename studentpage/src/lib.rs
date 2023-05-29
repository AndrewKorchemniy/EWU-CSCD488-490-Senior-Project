use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

mod components;
use components::footer::Footer;
use components::navbar::Navbar;

mod pages;
use pages::home::Home;
use pages::individual_report::IndividualReport;
use pages::page_not_found::PageNotFound;
use pages::requirements::Requirements;
use pages::team_report::TeamReport;

mod api;
mod stores;
//use api::{api_get_auth_config, OAuthClientConfigResponse};

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

    let logout = Callback::from(move |_: MouseEvent| {
        if let Err(err) = agent.start_login() {
            log!(format!("Failed to logout: {err}"));
        }
    });

    html! {
        <BrowserRouter>
            <div class="container-fluid mx-auto px-4 min-width">
                <div class="row">
                    <div class="col"> <Navbar logout={logout.clone()}/> </div>
                </div>
                <div class="row">
                    <div class="col"> <Switch<Route> render={switch} /> </div>
                </div>
                <div class="row">
                    <div class="col"> <Footer /> </div>
                </div>
            </div>
        </BrowserRouter>
        // TODO: Replace the above with the following when the API for OAuth is ready.
        // use components::msgbox::{MsgBox, MsgBoxVariant};
        // <BrowserRouter>
        //     <div class="container-fluid mx-auto px-4 min-width">
        //         <Failure>
        //             <div class="row">
        //                 <div class="col"> <Navbar /> </div>
        //             </div>
        //         </Failure>
        //         <Authenticated>
        //             <div class="row">
        //                 <div class="col"> <Navbar logout={logout.clone()}/> </div>
        //             </div>
        //             <div class="row">
        //                 <div class="col"> <Switch<Route> render={switch} /> </div>
        //             </div>
        //         </Authenticated>
        //         <NotAuthenticated>
        //             <MsgBox
        //                 title="Failed to authenticate"
        //                 text="Please try again later."
        //                 variant={MsgBoxVariant::Danger}>
        //             </MsgBox>
        //         </NotAuthenticated>
        //         <div class="row">
        //             <div class="col"> <Footer /> </div>
        //         </div>
        //     </div>
        // </BrowserRouter>
    }
}

const STYLESHEET: &str = include_str!("assets/main.css");

#[function_component(App)]
pub fn app() -> Html {
    // Set up global stylesheet
    let stylesheet = Style::new(STYLESHEET).unwrap();

    let config = Config {
        client_id: "client_id".to_string(),
        auth_url: "client_url".to_string(),
        token_url: "token_url".to_string(),
    };

    // TODO: DOES NOT WORK
    // cannot use Config within a sate
    // cannot use blocking api call within yew

    // The state for the OAuth2 config
    // let _config_state = use_state(|| config.clone());
    // let config_state = _config_state.clone();
    // let config_state_changes = Callback::from(move |config: OAuthClientConfigResponse| {
    //     _config_state.set(Config {
    //         client_id: config.client_id,
    //         auth_url: config.auth_url,
    //         token_url: config.token_url,
    //     });
    // });

    // wasm_bindgen_futures::spawn_local(async move {
    //     let result = api_get_auth_config().await;
    //     config_state_changes.emit(result);
    // });

    html! {
        //
        <div class={stylesheet}>
            <OAuth2 config={config} >
                <AppMain />
            </OAuth2>
        </div>
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
