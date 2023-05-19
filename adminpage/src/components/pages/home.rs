use gloo::console::log;
use gloo::file::File;
use web_sys::{Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::atoms::button::{Button, ButtonVariant};
use crate::stores::admin_store::AdminStore;

#[function_component(Home)]
pub fn home() -> Html {
    // Local session store.
    let (_, dispatch) = use_store::<AdminStore>();

    let onclick = dispatch.reduce_mut_callback_with(move |store, _| {
        log!("Clicked");
        store.testing = Some("Hello".to_string());
    });

    let upload_files = |files: Option<FileList>| {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        if result.len() > 0 {
            log!(result[0].name());
            log!(result[0].size());
            log!(result[0].raw_mime_type());
        }
    };

    let onchange = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        upload_files(input.files());
    });

    let ondrag = Callback::from(move |event: DragEvent| {
        let input: HtmlInputElement = event.target_unchecked_into();
        upload_files(input.files());
    });

    html! {
        <div class="card shadow">
            <div class="card-body">
                <form>
                    <div class="mb-3 col-12 col-xl-6">
                        <label
                            for="updateClasses"
                            class="mb-1 form-label text-light fs-4 fw-semibold">
                            { "Update Classes" }
                        </label>
                        <input class="form-control" type="file" id="updateClasses" {onchange} {ondrag}/>
                        <Button variant={ ButtonVariant::Danger }
                            onclick={ onclick }
                            class="mt-2"
                            label="Upload" />
                    </div>

                    <div class="mb-3 col-12 col-xl-6">
                        <label
                            for="updateTeams"
                            class="mb-1 form-label text-light fs-4 fw-semibold">
                            { "Update Teams" }
                        </label>
                        <input class="form-control" type="file" id="updateTeams"/>
                        <Button variant={ ButtonVariant::Danger } class="mt-2" label="Upload" />
                    </div>

                    <div class="col-12 col-xl-6 mb-2">
                        <label
                            for="download"
                            class="mb-1 form-label text-light fs-4 fw-semibold">
                            { "Download" }
                        </label>
                        <div class="dropdown" id="download">
                            <Button variant={ ButtonVariant::Light }
                                class="dropdown-toggle"
                                data_bs_toggle="dropdown"
                                label="Select Class" />
                            <ul class="dropdown-menu">
                                <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                                <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                                <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                            </ul>

                            <Button variant={ ButtonVariant::Light }
                                class="ms-2 dropdown-toggle"
                                data_bs_toggle="dropdown"
                                label="Select Team" />
                            <ul class="dropdown-menu">
                                <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                                <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                                <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                            </ul>
                        </div>
                    </div>
                    <Button variant={ ButtonVariant::Primary } label="Download" />
                </form>
            </div>
        </div>
    }
}
