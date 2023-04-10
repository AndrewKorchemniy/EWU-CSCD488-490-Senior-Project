use yew::prelude::*;

pub struct Navbar;
impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-light navbar-expand-md py-3" style="width: 95%;margin-right: auto;margin-left: auto;">
                <div class="container">
                    <a class="nav-link active" href="/"><span class="navbar-brand d-flex align-items-center"> { "Status Reports" } </span></a>
                    <button data-bs-toggle="collapse" class="navbar-toggler" data-bs-target="#navcol-1">
                        <span class="visually-hidden"> { "Toggle navigation" } </span>
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navcol-1">
                        <ul class="navbar-nav me-auto">
                            <li class="nav-item"><a class="nav-link active" href="/calendar"> { "Calendar" } </a></li>
                            <li class="nav-item"><a class="nav-link" href="#"> { "Requirements" } </a></li>
                            <li class="nav-item"><a class="nav-link" href="#"> { "Third Item" } </a></li>
                        </ul>
                        <button class="btn btn-primary" type="button"> { "Logout" } </button>
                    </div>
                </div>
            </nav>
        }
    }
}