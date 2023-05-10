use yew::prelude::*;

/// The [TimeAccounting] component provides a table for time accounting.
#[function_component(TimeAccounting)]
pub fn time_accounting() -> Html {
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
                                id="staturday" 
                                value="0"
                                style="width: 4rem" /> 
                        </td>                    
                    </tr>
                    <tr>
                        <td class="text-left"> { "Sunday" } </td>
                        <td> 
                            <input 
                                class="form-control form-control-sm" 
                                type="number" 
                                id="sunday" 
                                value="0"
                                style="width: 4rem" /> 
                        </td>
                    </tr>
                    <tr>
                        <td class="text-left"> { "Monday" } </td>
                        <td> 
                            <input 
                                class="form-control form-control-sm" 
                                type="number" 
                                id="monday" 
                                value="0"
                                style="width: 4rem" /> 
                        </td>                   
                    </tr>
                    <tr>
                        <td class="text-left"> { "Tuesday" } </td>
                        <td> 
                            <input 
                                class="form-control form-control-sm" 
                                type="number" 
                                id="tuesday" 
                                value="0"
                                style="width: 4rem" /> 
                        </td>                    
                    </tr>
                    <tr>
                        <td class="text-left"> { "Wednesday" } </td>
                        <td> 
                            <input 
                                class="form-control form-control-sm" 
                                type="number" 
                                id="wednesday" 
                                value="0"
                                style="width: 4rem" /> 
                        </td>                    
                    </tr>
                    <tr>
                        <td class="text-left"> { "Thursday" } </td>
                        <td> 
                            <input 
                                class="form-control form-control-sm" 
                                type="number" 
                                id="thursday" 
                                value="0"
                                style="width: 4rem" /> 
                        </td>                    
                    </tr>
                    <tr>
                        <td class="text-left"> { "Friday" } </td>
                        <td> 
                            <input 
                                class="form-control form-control-sm" 
                                type="number" 
                                id="friday" 
                                value="0"
                                style="width: 4rem" /> 
                        </td>                    
                    </tr>
                </tbody>
            </table>
        </div>
    }
}