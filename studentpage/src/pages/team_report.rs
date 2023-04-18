use yew::prelude::*;

#[function_component]
pub fn TeamReport() -> Html {
    html! {
        <div class="card shadow border-0">
            <div class="card-body">
                <h3 class="card-title"> { "Team Report" } </h3>
                <p class="card-subtitle>"> { "Consider the following four pairs of questions hierarchically. They are not the same question. If you think they are, then you are likely not using an appropriate breadth and depth of software-engineering thought. This course is a practical application of the aspects of product, process, and people. We are trying to account for everything: not just to create a good product, but also to learn from the process to improve the people. Reflect on the experience of the entire team collectively over this sprint. You do not need to account for all activities, just two that were representative of easiest and hardest. Use activity codes (e.g., A1) for specific references, but most of the response should be in sentence form."} </p>
                <p class="card-subtitle>"> { "For reference, understand relates to the comprehension of what need to be done; approach to how you think it should be solved; solve to implementing the actual solution; and evalutate to demonstrating to yourself and your team (if applicable) that the performance of your solution is consistent with everything else in the project. Remember The Cartoon from CS 350."} </p>
                <p class="card-subtitle>"> { "Everything on this form will be shared with all team members and the client"} </p>
                <hr/>
                <form>
                    <div class="row text-nowrap g-3">
                        <div class="col mb-3">
                            <label class="form-label"> { "Which aspects of the current work are the "} <b> {"easiest to understand?"} </b></label>
                            <textarea class="form-control"></textarea>
                        </div>
                        <div class="col mb-3">
                            <label class="form-label"> { "Which aspects of the current work are the "} <b> {"hardest to understand?"} </b></label>
                            <textarea class="form-control"></textarea>
                        </div>
                    </div>
                    <div class="row text-nowrap g-3">
                        <div class="col mb-3">
                            <label class="form-label"> { "Which aspects of the current work are the "} <b> {"easiest to approach?"} </b></label>
                            <textarea class="form-control"></textarea>
                        </div>
                        <div class="col mb-3">
                            <label class="form-label"> { "Which aspects of the current work are the "} <b> {"hardest to approach?"} </b></label>
                            <textarea class="form-control"></textarea>
                        </div>
                    </div>
                    <div class="row text-nowrap g-3">
                        <div class="col mb-3">
                             <label class="form-label"> { "Which aspects of the current work are the "} <b> {"easiest to solve?"} </b></label>
                             <textarea class="form-control"></textarea>
                        </div>
                        <div class="col mb-3">
                           <label class="form-label"> { "Which aspects of the current work are the "} <b> {"hardest to solve?"} </b></label>
                           <textarea class="form-control"></textarea>
                        </div>
                    </div>
                    <div class="row text-nowrap g-3">
                        <div class="col mb-3">
                            <label class="form-label"> { "Which aspects of the current work are the "} <b> {"easiest to evaluate?"} </b></label>
                            <textarea class="form-control"></textarea>
                        </div>
                        <div class="col mb-3">
                            <label class="form-label"> { "Which aspects of the current work are the "} <b> {"hardest to eavluate?"} </b></label>
                            <textarea class="form-control"></textarea>
                        </div>
                    </div>
                    <div class="row text-nowrap">
                        <div class="col mb-3">
                            <label class="form-label"> { "How far along (as a percent) do you feel you are toward the final goal? Does this pace seem likely to succeed?" }</label>
                            <textarea class="form-control"></textarea>
                        </div>
                    </div>
                    <div class="row text-nowrap">
                        <div class="col mb-3">
                            <label class="form-label"> { "Did you meet with your client this week? If not, when was the last time?" }</label>
                            <textarea class="form-control"></textarea>
                        </div>
                    </div>
                    <div class="row text-nowrap">
                        <div class="col mb-3">
                            <label class="form-label"> { "Are there any issues, concern, or comments about the project?" }</label>
                            <textarea class="form-control"></textarea>
                        </div>
                    </div>
                    <div class="row text-nowrap">
                        <div class="col">
                            <button class="btn btn-primary" type="submit"> {"Submit"} </button>
                        </div>
                    </div>
                </form>
            </div>
        </div>
    }
}
