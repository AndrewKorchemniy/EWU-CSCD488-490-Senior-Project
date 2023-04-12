use yew::prelude::*;

#[function_component]
pub fn PageNotFound() -> Html {
    html! {
        <section class="hero is-danger is-bold is-large">
            <div class="hero-body">
                <h1 class="title">
                    { "Page not found" }
                </h1>
                <h2 class="subtitle">
                    { "Page page does not seem to exist" }
                </h2>
            </div>
        </section>
    }
}