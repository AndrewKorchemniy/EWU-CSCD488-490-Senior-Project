use yew::prelude::*;

/// The [ActivityAccounting] component provides a table for activity accounting.
#[function_component(ActivityAccounting)]
pub fn acitvity_accounting() -> Html {
    html! {
        <div class="rounded me-auto table-responsive mt-2">
            <table class="table table-hover table-borderless mb-0 border-0">
                <thead>
                    <th class="text-center text-secondary text-nowrap"> { "Code" } </th>
                    <th class="text-center text-secondary text-nowrap"> { "Title" } </th>
                    <th class="text-center text-secondary text-nowrap"> { "Description" } </th>
                    <th class="text-center text-secondary text-nowrap"> { "Sprints" } </th>
                </thead>
                <tbody>
                    <tr>
                        <td class="text-left"> { "A1" } </td>
                        <td> 
                            <input class="form-control form-control-sm" type="text" rows="1" />
                        </td>
                        <td> 
                            <input class="form-control form-control-sm" type="text" rows="1" cols="20" />
                        </td>
                        <td> 
                            <input class="form-control form-control-sm" type="number" value="0" style="width: 4rem"/> 
                        </td>                    
                    </tr>
                    <tr>
                        <td class="text-left"> { "A2" } </td>
                        <td> 
                            <input class="form-control form-control-sm" type="text" rows="1" />
                        </td>
                        <td> 
                            <input class="form-control form-control-sm" type="text" rows="1" cols="20" />
                        </td>
                        <td> 
                            <input class="form-control form-control-sm" type="number" value="0" style="width: 4rem"/> 
                        </td>                    
                    </tr>
                    <tr>
                        <td class="text-left"> { "A3" } </td>
                        <td> 
                            <input class="form-control form-control-sm" type="text" rows="1" />
                        </td>
                        <td> 
                            <input class="form-control form-control-sm" type="text" rows="1" cols="20" />
                        </td>
                        <td> 
                            <input class="form-control form-control-sm" type="number" value="0" style="width: 4rem"/> 
                        </td>                    
                    </tr>
                </tbody>
            </table>
        </div>
    }
}