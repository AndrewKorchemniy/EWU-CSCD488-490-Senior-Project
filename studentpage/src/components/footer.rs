use yew::prelude::*;

pub struct Footer;
impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="text-left text-muted mt-2 px-2">
                <p> { "© 2023 Dan Tappan" } </p>
            </footer>   
        }
    }
}