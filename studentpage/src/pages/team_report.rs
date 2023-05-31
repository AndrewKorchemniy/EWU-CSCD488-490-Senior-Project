use gloo_utils::window;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::form::Form;
use crate::components::instructions::Instructions;
use crate::components::modal::Modal;
use crate::components::range::Range;
use crate::components::text_area::TextAreaValidation;
use crate::components::text_area::{TextArea, TextAreaVariant};

use crate::stores::team_store::{validate, TeamStore};

#[function_component(TeamReport)]
pub fn team_report() -> Html {
    let document = window();

    // Local session store.
    let (store, dispatch) = use_store::<TeamStore>();

    // State for displaying range value.
    let _range_state = use_state(|| {
        if store.completion_percent.is_none() {
            AttrValue::from("")
        } else {
            AttrValue::from(format!("{}%", store.completion_percent.as_deref().unwrap()))
        }
    });
    let range_state = _range_state.clone();
    let range_changed = Callback::from(move |value: AttrValue| {
        _range_state.set(AttrValue::from(format!("{}%", value)));
    });

    // Validation state for each textarea.
    let _understand_easy_state = use_state(|| TextAreaValidation::None);
    let understand_easy_state = _understand_easy_state.clone();
    let understand_easy_state_changes = Callback::from(move |value: TextAreaValidation| {
        _understand_easy_state.set(value);
    });
    let _understand_hard_state = use_state(|| TextAreaValidation::None);
    let understand_hard_state = _understand_hard_state.clone();
    let understand_hard_state_changes = Callback::from(move |value: TextAreaValidation| {
        _understand_hard_state.set(value);
    });
    let _approach_easy_state = use_state(|| TextAreaValidation::None);
    let approach_easy_state = _approach_easy_state.clone();
    let approach_easy_state_changes = Callback::from(move |value: TextAreaValidation| {
        _approach_easy_state.set(value);
    });
    let _approach_hard_state = use_state(|| TextAreaValidation::None);
    let approach_hard_state = _approach_hard_state.clone();
    let approach_hard_state_changes = Callback::from(move |value: TextAreaValidation| {
        _approach_hard_state.set(value);
    });
    let _solve_easy_state = use_state(|| TextAreaValidation::None);
    let solve_easy_state = _solve_easy_state.clone();
    let solve_easy_state_changes = Callback::from(move |value: TextAreaValidation| {
        _solve_easy_state.set(value);
    });
    let _solve_hard_state = use_state(|| TextAreaValidation::None);
    let solve_hard_state = _solve_hard_state.clone();
    let solve_hard_state_changes = Callback::from(move |value: TextAreaValidation| {
        _solve_hard_state.set(value);
    });
    let _evaluate_easy_state = use_state(|| TextAreaValidation::None);
    let evaluate_easy_state = _evaluate_easy_state.clone();
    let evaluate_easy_state_changes = Callback::from(move |value: TextAreaValidation| {
        _evaluate_easy_state.set(value);
    });
    let _evaluate_hard_state = use_state(|| TextAreaValidation::None);
    let evaluate_hard_state = _evaluate_hard_state.clone();
    let evaluate_hard_state_changes = Callback::from(move |value: TextAreaValidation| {
        _evaluate_hard_state.set(value);
    });
    let _pace_succeed_state = use_state(|| TextAreaValidation::None);
    let pace_succeed_state = _pace_succeed_state.clone();
    let pace_succeed_state_changes = Callback::from(move |value: TextAreaValidation| {
        _pace_succeed_state.set(value);
    });
    let _client_meeting_state = use_state(|| TextAreaValidation::None);
    let client_meeting_state = _client_meeting_state.clone();
    let client_meeting_state_changes = Callback::from(move |value: TextAreaValidation| {
        _client_meeting_state.set(value);
    });

    // Callbacks to store changes and perform proactive validation.
    let cloned_understand_easy = understand_easy_state_changes.clone();
    let understand_easy_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.understand_easy = Some(value);
        validate(&store.understand_easy, &cloned_understand_easy);
    });
    let cloned_understand_hard = understand_hard_state_changes.clone();
    let understand_hard_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.understand_hard = Some(value);
        validate(&store.understand_hard, &cloned_understand_hard);
    });
    let cloned_approach_easy = approach_easy_state_changes.clone();
    let approach_easy_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.approach_easy = Some(value);
        validate(&store.approach_easy, &cloned_approach_easy);
    });
    let cloned_approach_hard = approach_hard_state_changes.clone();
    let approach_hard_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.approach_hard = Some(value);
        validate(&store.approach_hard, &cloned_approach_hard);
    });
    let cloned_solve_easy = solve_easy_state_changes.clone();
    let solve_easy_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.solve_easy = Some(value);
        validate(&store.solve_easy, &cloned_solve_easy);
    });
    let cloned_solve_hard = solve_hard_state_changes.clone();
    let solve_hard_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.solve_hard = Some(value);
        validate(&store.solve_hard, &cloned_solve_hard);
    });
    let cloned_evaluate_easy = evaluate_easy_state_changes.clone();
    let evaluate_easy_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.evaluate_easy = Some(value);
        validate(&store.evaluate_easy, &cloned_evaluate_easy);
    });
    let cloned_evaluate_hard = evaluate_hard_state_changes.clone();
    let evaluate_hard_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.evaluate_hard = Some(value);
        validate(&store.evaluate_hard, &cloned_evaluate_hard);
    });
    let completion_percent_onchange = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value = target.value().trim().to_string();
        store.completion_percent = Some(value);
    });
    let cloned_pace_succeed = pace_succeed_state_changes.clone();
    let pace_succeed_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().trim().to_lowercase();
        store.pace_succeed = Some(value);
        validate(&store.pace_succeed, &cloned_pace_succeed);
    });
    let cloned_client_meeting = client_meeting_state_changes.clone();
    let client_meeting_onchange = dispatch.reduce_mut_callback_with(move |store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().to_string();
        store.client_meeting = Some(value);
        validate(&store.client_meeting, &cloned_client_meeting);
    });
    let issues_comments_onchange = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let target: HtmlTextAreaElement = event.target_unchecked_into();
        let value = target.value().to_string();
        store.issues_comments = Some(value);
    });

    // Validate all fields on submit.
    let validate_submit = move |store: &mut TeamStore| {
        let mut valid = true;
        valid = validate(&store.understand_easy, &understand_easy_state_changes) && valid;
        valid = validate(&store.understand_hard, &understand_hard_state_changes) && valid;
        valid = validate(&store.approach_easy, &approach_easy_state_changes) && valid;
        valid = validate(&store.approach_hard, &approach_hard_state_changes) && valid;
        valid = validate(&store.solve_easy, &solve_easy_state_changes) && valid;
        valid = validate(&store.solve_hard, &solve_hard_state_changes) && valid;
        valid = validate(&store.evaluate_easy, &evaluate_easy_state_changes) && valid;
        valid = validate(&store.evaluate_hard, &evaluate_hard_state_changes) && valid;
        valid = validate(&store.pace_succeed, &pace_succeed_state_changes) && valid;
        valid = validate(&store.client_meeting, &client_meeting_state_changes) && valid;
        valid
    };

    // Callback for submitting the form. Triggers client-side validation.
    let onsubmit = dispatch.reduce_mut_callback_with(move |store, event: MouseEvent| {
        event.prevent_default();
        if validate_submit(store) {
            document.location().set_href("team-report/submit").ok();
        }
    });

    html! {
        <Form>
            <Instructions
                text="Consider the following four pairs of questions hierarchically. They are not the same question. If you think they are, then you are likely not using an appropriate breadth and depth of software-engineering thought. This course is a practical application of the aspects of product, process, and people. We are trying to account for everything: not just to create a good product, but also to learn from the process to improve the people. Reflect on the experience of the entire team collectively over this sprint. You do not need to account for all activities, just two that were representative of easiest and hardest. Use activity codes (e.g., A1) for specific references, but most of the response should be in sentence form. <br/> <br/> For reference, understand relates to the comprehension of what need to be done; approach to how you think it should be solved; solve to implementing the actual solution; and evalutate to demonstrating to yourself and your team (if applicable) that the performance of your solution is consistent with everything else in the project. Remember The Cartoon from CS 350.
                <hr class='mb-0'/>"/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to understand</b>?"
                id="understand-easy"
                value={ store.understand_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ understand_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &understand_easy_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to understand</b>?"
                id="understand-hard"
                value={ store.understand_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ understand_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &understand_hard_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to approach</b>?"
                id="approach-easy"
                value={ store.approach_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ approach_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &approach_easy_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to approach</b>?"
                id="approach-hard"
                value={ store.approach_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ approach_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &approach_hard_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to solve</b>?"
                id="solve-easy"
                value={ store.solve_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ solve_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &solve_easy_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to solve</b>?"
                id="solve-hard"
                value={ store.solve_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ solve_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &solve_hard_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to evaluate</b>?"
                id="evaluate-easy"
                value={ store.evaluate_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ evaluate_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &evaluate_easy_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to evaluate</b>?"
                id="evaluate-hard"
                value={ store.evaluate_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ evaluate_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &evaluate_hard_onchange }/>
            <Range
                label={format!(
                    "How far along (as a percent) do you feel you are toward the final goal? <b>{}</b>",
                    &*range_state)}
                id="completion-percent"
                initial_value={ store.completion_percent.as_deref().unwrap_or("-1").to_string() }
                handle_oninput={range_changed}
                onchange={ &completion_percent_onchange }/>
            <TextArea
                label="Does this pace seem likely to succeed?"
                id="pace-succeed"
                value={ store.pace_succeed.as_deref().unwrap_or_default().to_string() }
                is_valid={ pace_succeed_state }
                onchange={ &pace_succeed_onchange }/>
            <TextArea
                label="Did you meet with your client this week? If not, when was the last time?"
                id="client-meeting"
                value={ store.client_meeting.as_deref().unwrap_or_default().to_string() }
                is_valid={ client_meeting_state }
                onchange={ &client_meeting_onchange }/>
            <TextArea
                label="Are there any issues, concern, or comments about the project?"
                id="issues-comments"
                value={ store.issues_comments.as_deref().unwrap_or_default().to_string() }
                onchange={ &issues_comments_onchange }/>
            <Button
                variant={ ButtonVariant::Danger }
                label="Submit"
                class="mt-3 col-auto ms-2"
                data_bs_toggle="modal"
                data_bs_target="#confirm" />
            <Modal
                id="confirm"
                title="Are you sure?"
                body="You can only submit once."
                action_label="Submit"
                action_button_type="submit"
                onclick={ onsubmit } />
        </Form>
    }
}
