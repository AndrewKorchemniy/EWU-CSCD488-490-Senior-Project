use gloo::console::log;
use gloo::file::File;
use web_sys::{Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::download::Download;
use crate::components::modal::Modal;
use crate::stores::admin_store::AdminStore;

#[function_component(Home)]
pub fn home() -> Html {
    // Local session store.
    let (_, dispatch) = use_store::<AdminStore>();

    let onclick = dispatch.reduce_mut_callback_with(move |store, _| {
        log!("Clicked");
        store.testing = Some("Hello".to_string());
    });

    // Closure to parse and upload file content.
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

        // TODO: Call the api to send the file contents.
        // File upload example: https://github.com/yewstack/yew/tree/master/examples/file_upload 
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
                <form onsubmit={ Callback::from(|event: SubmitEvent| {
                    event.prevent_default(); // Prevent the defualt form submission behavior.
                })}>
                    // Upload class csv file.
                    <div class="mb-3 col-12 col-xl-6">
                        <label
                            for="updateClasses"
                            class="form-label text-light fs-4 fw-semibold">
                            { "Update Classes" }
                        </label>
                        <input
                            class="form-control"
                            type="file"
                            id="updateClasses"
                            onchange={&onchange}
                            ondrag={&ondrag}/>
                        <Button
                            variant={ ButtonVariant::Danger }
                            label="Upload"
                            class="mt-3"
                            data_bs_toggle="modal"
                            data_bs_target="#confirm-class-upload" />
                        <Modal
                            id="confirm-class-upload"
                            title="Are you sure?"
                            body="This action is irreversible."
                            action_label="Upload"
                            onclick={ &onclick } />
                    </div>

                    // Upload students csv file.
                    <div class="mb-3 col-12 col-xl-6">
                        <label
                            for="updateStudents"
                            class="form-label text-light fs-4 fw-semibold">
                            { "Update Students" }
                        </label>
                        <input
                            class="form-control"
                            type="file"
                            id="updateStudents"/>
                        <Button
                            variant={ ButtonVariant::Danger }
                            label="Upload"
                            class="mt-3"
                            data_bs_toggle="modal"
                            data_bs_target="#confirm-students-upload" />
                        <Modal
                            id="confirm-students-upload"
                            title="Are you sure?"
                            body="This action is irreversible."
                            action_label="Upload"
                            onclick={ &onclick } />
                    </div>

                    // Upload teams csv file.
                    <div class="mb-3 col-12 col-xl-6">
                        <label
                            for="updateTeams"
                            class="form-label text-light fs-4 fw-semibold">
                            { "Update Teams" }
                        </label>
                        <input class="form-control" type="file" id="updateTeams"/>
                        <Button
                            variant={ ButtonVariant::Danger }
                            label="Upload"
                            class="mt-3"
                            data_bs_toggle="modal"
                            data_bs_target="#confirm-teams-upload" />
                        <Modal
                            id="confirm-teams-upload"
                            title="Are you sure?"
                            body="This action is irreversible."
                            action_label="Upload"
                            onclick={ &onclick } />
                    </div>

                    // Download submissions group.
                    <Download />
                </form>
            </div>
        </div>
    }
}
