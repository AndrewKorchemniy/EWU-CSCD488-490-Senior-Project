use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::individual_store::IndividualStore;

/// The properties for the [TimeAccounting] component.
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    /// The store to save changes to.
    pub store: Rc<IndividualStore>,
    /// The dispatcher to save changes with.
    pub dispatch: Dispatch<IndividualStore>,
}

/// The [TimeAccounting] component provides a table for time accounting.
/// All changes are stored into the provided store.
#[function_component(TimeAccounting)]
pub fn time_accounting(props: &Props) -> Html {
    // Callbacks to store changes.
    let dispatch = props.dispatch.clone();
    let store = props.store.clone();
    let saturday_hours_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.saturday_hours = Some(value.parse::<i32>().unwrap_or_default());
    });
    let sunday_hours_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.sunday_hours = Some(value.parse::<i32>().unwrap_or_default());
    });
    let monday_hours_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.monday_hours = Some(value.parse::<i32>().unwrap_or_default());
    });
    let tuesday_hours_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.tuesday_hours = Some(value.parse::<i32>().unwrap_or_default());
    });
    let wednesday_hours_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.wednesday_hours = Some(value.parse::<i32>().unwrap_or_default());
    });
    let thursday_hours_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.thursday_hours = Some(value.parse::<i32>().unwrap_or_default());
    });
    let friday_hours_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.friday_hours = Some(value.parse::<i32>().unwrap_or_default());
    });

    // Render empty value if None.
    let some_or_empty = |value: Option<i32>| -> String {
        if value.is_some() {
            value.unwrap().to_string()
        } else {
            String::from("")
        }
    };

    html! {
        <div class="rounded me-auto table-responsive mt-2">
            <table class="table table-hover table-borderless mb-0 border-0">
                <thead>
                    <th class="text-center text-secondary text-nowrap"> { "Day" } </th>
                    <th class="text-center text-secondary text-nowrap"> { "Hours" } </th>
                </thead>
                <tbody>
                    <tr>
                        <td class="text-left"> { "Saturday" } </td>
                        <td>
                            <input
                                class="form-control form-control-sm"
                                type="number"
                                min="0"
                                max="100"
                                value={ some_or_empty(store.saturday_hours) }
                                onchange={ saturday_hours_onchange }
                                style="width: 4rem" />
                        </td>
                    </tr>
                    <tr>
                        <td class="text-left"> { "Sunday" } </td>
                        <td>
                            <input
                                class="form-control form-control-sm"
                                type="number"
                                min="0"
                                max="100"
                                value={ some_or_empty(store.sunday_hours) }
                                onchange={ sunday_hours_onchange }
                                style="width: 4rem" />
                        </td>
                    </tr>
                    <tr>
                        <td class="text-left"> { "Monday" } </td>
                        <td>
                            <input
                                class="form-control form-control-sm"
                                type="number"
                                min="0"
                                max="100"
                                value={ some_or_empty(store.monday_hours) }
                                onchange={ monday_hours_onchange }
                                style="width: 4rem" />
                        </td>
                    </tr>
                    <tr>
                        <td class="text-left"> { "Tuesday" } </td>
                        <td>
                            <input
                                class="form-control form-control-sm"
                                type="number"
                                min="0"
                                max="100"
                                value={ some_or_empty(store.tuesday_hours) }
                                onchange={ tuesday_hours_onchange }
                                style="width: 4rem" />
                        </td>
                    </tr>
                    <tr>
                        <td class="text-left"> { "Wednesday" } </td>
                        <td>
                            <input
                                class="form-control form-control-sm"
                                type="number"
                                min="0"
                                max="100"
                                value={ some_or_empty(store.wednesday_hours) }
                                onchange={ wednesday_hours_onchange }
                                style="width: 4rem" />
                        </td>
                    </tr>
                    <tr>
                        <td class="text-left"> { "Thursday" } </td>
                        <td>
                            <input
                                class="form-control form-control-sm"
                                type="number"
                                min="0"
                                max="100"
                                value={ some_or_empty(store.thursday_hours) }
                                onchange={ thursday_hours_onchange }
                                style="width: 4rem" />
                        </td>
                    </tr>
                    <tr>
                        <td class="text-left"> { "Friday" } </td>
                        <td>
                            <input
                                class="form-control form-control-sm"
                                type="number"
                                min="0"
                                max="100"
                                value={ some_or_empty(store.friday_hours) }
                                onchange={ friday_hours_onchange }
                                style="width: 4rem" />
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
