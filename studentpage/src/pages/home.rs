use yew::prelude::*;


pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="table-responsive border rounded text-nowrap shadow" style="margin-left: auto; margin-right: auto">
                <table class="table table-striped">

                    <thead>
                        <tr>
                            <th class="table-warning text-center text-secondary" style="width: 5%;"> {"Sprint"} </th>
                            <th class="table-warning text-left text-secondary" style="width: 0%;"> {"Due"} </th>
                            <th class="table-warning text-center text-secondary" style="width: 0%;"> {"Team Report"} </th>
                            <th class="table-warning text-center text-secondary" style="width: 0%;"> {"Individual Report"} </th>
                            <th class="table-warning text-secondary"></th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"1"}</td>
                            <td style="width: 0%;">{"Jan 13"}</td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr class="secondary">
                            <td class="text-center" style="width: 5%;">{"2"}</td>
                            <td style="width: 0%;">{"Jan 20"}</td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"3"}</td>
                            <td>{"Jan 27"}</td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"4"}</td>
                            <td style="width: 0%;">{"Feb 3"}</td>
                            <td class="text-center" style="color: tomato"><i class="fas fa-xmark fa-xl"></i></td>
                            <td class="text-center" style="color: tomato"><i class="fas fa-xmark fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"5"}</td>
                            <td style="width: 0%;">{"Feb 10"}</td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"6"}</td>
                            <td style="width: 0%;">{"Feb 17"}</td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td class="text-center" style="color: limegreen"><i class="fas fa-check fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"7"}</td>
                            <td style="width: 0%;">{"Feb 24"}</td>
                            <td class="text-center" style="color: dodgerblue">
                                <a href="/team-report"><i class="fas fa-arrow-right fa-xl"></i></a>
                            </td>
                            <td class="text-center" style="color: dodgerblue">
                            <a href="/individual-report"><i class="fas fa-arrow-right fa-xl"></i></a>
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"8"}</td>
                            <td style="width: 0%;">{"Mar 3"}</td>
                            <td class="text-center" style="color: darkgray"><i class="fas fa-minus fa-xl"></i></td>
                            <td class="text-center" style="color: darkgray"><i class="fas fa-minus fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"9"}</td>
                            <td style="width: 0%;">{"Mar 10"}</td>
                            <td class="text-center" style="color: darkgray"><i class="fas fa-minus fa-xl"></i></td>
                            <td class="text-center" style="color: darkgray"><i class="fas fa-minus fa-xl"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"10"}</td>
                            <td style="width: 0%;">{"Mar 17"}</td>
                            <td class="text-center" style="color: darkgray"><i class="fas fa-minus fa-xl"></i></td>
                            <td class="text-center" style="color: darkgray"><i class="fas fa-minus fa-xl"></i></td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
    }
}