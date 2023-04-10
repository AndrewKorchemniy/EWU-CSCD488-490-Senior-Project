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
            <p class="text-muted my-2 d-flex p-2"> { "© 2023 Dan Tappan" } </p>
        }
    }
}