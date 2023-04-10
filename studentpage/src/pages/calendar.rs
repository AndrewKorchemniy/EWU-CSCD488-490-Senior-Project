use yew::prelude::*;

pub struct Calendar;
impl Component for Calendar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="table-responsive border rounded text-nowrap" style="box-shadow: 0px 6px 10px var(--bs-gray-400);margin-left: auto;margin-right: auto;width: 95%;">
                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th class="table-warning" style="text-align: center;color: rgb(85,85,85);width: 5%;"> {"Sprint"} </th>
                            <th class="table-warning" style="color: rgb(85,85,85);text-align: left;width: 0%;"> {"Due"} </th>
                            <th class="table-warning" style="color: rgb(85,85,85);text-align: center;width: 0%;"> {"Team Report"} </th>
                            <th class="table-warning text-center" style="color: rgb(85,85,85);width: 0%;"> {"Individual Report"} </th>
                            <th class="table-warning" style="color: rgb(85,85,85);"></th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"1"}</td>
                            <td style="width: 0%;">{"Jan 13"}</td>
                            <td style="color: rgb(43,213,0);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(43,213,0);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"2"}</td>
                            <td style="width: 0%;">{"Jan 20"}</td>
                            <td style="color: rgb(43,213,0);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(43,213,0);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"3"}</td>
                            <td>{"Jan 27"}</td>
                            <td style="color: rgb(255,0,0);text-align: center;width: 0%;"><i class="fas fa-times" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(255,0,0);"><i class="fas fa-times" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"4"}</td>
                            <td style="width: 0%;">{"Feb 3"}</td>
                            <td style="color: rgb(43,213,0);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(43,213,0);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"5"}</td>
                            <td style="width: 0%;">{"Feb 10"}</td>
                            <td style="color: rgb(43,213,0);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(43,213,0);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"6"}</td>
                            <td style="width: 0%;">{"Feb 17"}</td>
                            <td style="color: rgb(43,213,0);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(43,213,0);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"7"}</td>
                            <td style="width: 0%;">{"Feb 24"}</td>
                            <td style="color: rgb(0,128,255);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(0,128,255);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"8"}</td>
                            <td style="width: 0%;">{"Mar 3"}</td>
                            <td style="color: rgb(143,143,143);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(143,143,143);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"9"}</td>
                            <td style="width: 0%;">{"Mar 10"}</td>
                            <td style="color: rgb(143,143,143);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(143,143,143);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="text-center" style="width: 5%;">{"10"}</td>
                            <td style="width: 0%;">{"Mar 17"}</td>
                            <td style="color: rgb(143,143,143);text-align: center;width: 0%;"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td class="text-center" style="width: 0%;color: rgb(143,143,143);"><i class="fas fa-check" style="font-size: 23px;"></i></td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
    }
}