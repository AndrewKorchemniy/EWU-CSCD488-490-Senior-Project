use yew::prelude::*;

/// Properties for [Calendar]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

/// The [Calendar] component provides a table of sprints.
/// Each sprint has an associated due date, team report, and individual report.
/// This cwomponent mostly serves as a styled container for the [Sprint] component.
#[function_component(Calendar)]
pub fn calendar(props: &Props) -> Html {
    html! {
        <div class="border rounded shadow mx-auto">
            <table class="table table-striped mb-0">
                <thead>
                    <tr>
                        <th class="table-warning text-center text-secondary text-nowrap" style="width: 1%"> {"Sprint"} </th>
                        <th class="table-warning text-left text-secondary text-nowrap" style="width: 1%"> {"Due"} </th>
                        <th class="table-warning text-center text-secondary text-nowrap" style="width: 1%"> {"Individual"} </th>
                        <th class="table-warning text-center text-secondary text-nowrap" style="width: 1%"> {"Team"} </th>
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
