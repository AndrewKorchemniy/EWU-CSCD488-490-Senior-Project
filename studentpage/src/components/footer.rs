use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer class="text-left text-muted mt-2 px-2">
            <p> { "© 2023 Dan Tappan" } </p>
        </footer>   
    }
}  