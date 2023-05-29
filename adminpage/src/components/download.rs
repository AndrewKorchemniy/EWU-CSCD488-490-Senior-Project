use yew::prelude::*;

use crate::components::button::{Button, ButtonVariant};

/// The [Download] component provides a way to download class sprint submissions.
#[function_component(Download)]
pub fn download() -> Html {
    html! {
        <div class="col-12 col-xl-6">
            <label
                for="download"
                class="form-label text-light fs-4 fw-semibold">
                { "Download" }
            </label>
            <div class="dropdown" id="download">
                <Button
                    variant={ ButtonVariant::Light }
                    class="dropdown-toggle"
                    data_bs_toggle="dropdown"
                    label="Select Class" />
                <ul class="dropdown-menu">
                    <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                    <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                    <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                </ul>
                <Button
                    variant={ ButtonVariant::Light }
                    class="ms-3 dropdown-toggle"
                    data_bs_toggle="dropdown"
                    label="Select Sprint" />
                <ul class="dropdown-menu">
                    <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                    <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                    <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                </ul>
            </div>
            <Button variant={ ButtonVariant::Primary } class="mt-3" label="Download" />
        </div>
    }
}
