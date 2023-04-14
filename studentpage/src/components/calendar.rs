use yew::prelude::*;

// Properties for [Calendar]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children
}

// The [Calendar] component provides a table of sprints.
// Each sprint has an associated due date, team report, and individual report.
// This component mostly serves as a styled container for the [Sprint] component.
#[function_component]
pub fn Calendar(props: &Props) -> Html {
    html! {
        <div class="border rounded shadow mx-auto">
            <table class="table table-striped text-nowrap mb-0">
                <thead>
                    <tr>
                        <th class="table-warning text-center text-secondary" style="width: 1%"> {"Sprint"} </th>
                        <th class="table-warning text-left text-secondary" style="width: 1%"> {"Due"} </th>
                        <th class="table-warning text-center text-secondary" style="width: 1%"> {"Individual Report"} </th>
                        <th class="table-warning text-center text-secondary" style="width: 1%"> {"Team Report"} </th>
                        <th class="table-warning text-secondary"></th>
                    </tr>
                </thead>
                <tbody>
                    { for props.children.iter() }
                </tbody>
            </table>
        </div>
    }
}