use yew::prelude::*;

/// Properties for [Calendar]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

/// The [Calendar] component provides a styled table of sprints.
/// Each sprint has an associated due date, team report, and individual report.
#[function_component(Calendar)]
pub fn calendar(props: &Props) -> Html {
    html! {
        <div class="border rounded shadow mx-auto table-responsive">
            <table class="table table-hover table-striped mb-0">
                <thead>
                    <tr>
                        <th class="table-warning text-center text-secondary text-nowrap"> {"Sprint"} </th>
                        <th class="table-warning text-left text-secondary text-nowrap"> {"Due"} </th>
                        <th class="table-warning text-center text-secondary text-nowrap"> {"Individual"} </th>
                        <th class="table-warning text-center text-secondary text-nowrap"> {"Team"} </th>
                        <th class="table-warning w-100"></th>
                    </tr>
                </thead>
                <tbody>
                    { for props.children.iter() }
                </tbody>
            </table>
        </div>
    }
}
