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
        <div class="table-responsive border rounded text-nowrap shadow" style="margin-left: auto; margin-right: auto">
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th class="table-warning text-center text-secondary" style="width: 0%;"> {"Sprint"} </th>
                        <th class="table-warning text-left text-secondary" style="width: 0%;"> {"Due"} </th>
                        <th class="table-warning text-center text-secondary" style="width: 0%;"> {"Team Report"} </th>
                        <th class="table-warning text-center text-secondary" style="width: 0%;"> {"Individual Report"} </th>
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