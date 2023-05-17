use gloo::console::log;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::atoms::button::{Button, ButtonVariant};
use crate::components::atoms::instructions::Instructions;
use crate::components::atoms::range::Range;
use crate::components::atoms::text_area::TextAreaValidation;
use crate::components::atoms::text_area::{TextArea, TextAreaVariant};
use crate::components::molecules::team_form::TeamForm;

use crate::stores::team_store::TeamStore;

#[function_component(TeamReport)]
pub fn team_report() -> Html {
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

    // Callback to store changes in session storage.
    let store_onchange = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let target = event.target_unchecked_into::<HtmlTextAreaElement>();
        let id = target.id();
        let value = target.value().trim().to_string(); // Trim whitespace before storing.

        let as_option = |value: String| -> Option<String> {
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        };

        match id.as_str() {
            "understand-easy" => store.understand_easy = as_option(value),
            "understand-hard" => store.understand_hard = as_option(value),
            "approach-easy" => store.approach_easy = as_option(value),
            "approach-hard" => store.approach_hard = as_option(value),
            "solve-easy" => store.solve_easy = as_option(value),
            "solve-hard" => store.solve_hard = as_option(value),
            "evaluate-easy" => store.evaluate_easy = as_option(value),
            "evaluate-hard" => store.evaluate_hard = as_option(value),
            "completion-percent" => store.completion_percent = as_option(value),
            "pace-succeed" => store.pace_succeed = as_option(value),
            "client-meeting" => store.client_meeting = as_option(value),
            "issues-comments" => store.issues_comments = as_option(value),
            _ => (),
        }
    });

    // Client-side validation.
    let validate = |text: &Option<String>, state: &Callback<TextAreaValidation>| -> bool {
        match text {
            Some(text) => {
                if text.len() < 3 {
                    state.emit(TextAreaValidation::Invalid);
                    false
                } else {
                    state.emit(TextAreaValidation::Valid);
                    true
                }
            }
            None => {
                state.emit(TextAreaValidation::Invalid);
                false
            }
        }
    };

    let validate = move |store: &mut TeamStore| {
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
    let onsubmit = dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
        event.prevent_default();
        log!(validate(store));
        // TODO: submit to server
    });

    html! {
        <TeamForm onsubmit={ onsubmit }>
            <Instructions
                text="Consider the following four pairs of questions hierarchically. They are not the same question. If you think they are, then you are likely not using an appropriate breadth and depth of software-engineering thought. This course is a practical application of the aspects of product, process, and people. We are trying to account for everything: not just to create a good product, but also to learn from the process to improve the people. Reflect on the experience of the entire team collectively over this sprint. You do not need to account for all activities, just two that were representative of easiest and hardest. Use activity codes (e.g., A1) for specific references, but most of the response should be in sentence form. <br/> <br/> For reference, understand relates to the comprehension of what need to be done; approach to how you think it should be solved; solve to implementing the actual solution; and evalutate to demonstrating to yourself and your team (if applicable) that the performance of your solution is consistent with everything else in the project. Remember The Cartoon from CS 350.
                <hr class='mb-0'/>"/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to understand</b>?"
                id="understand-easy"
                value={ store.understand_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ understand_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to understand</b>?"
                id="understand-hard"
                value={ store.understand_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ understand_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to approach</b>?"
                id="approach-easy"
                value={ store.approach_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ approach_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to approach</b>?"
                id="approach-hard"
                value={ store.approach_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ approach_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to solve</b>?"
                id="solve-easy"
                value={ store.solve_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ solve_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to solve</b>?"
                id="solve-hard"
                value={ store.solve_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ solve_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to evaluate</b>?"
                id="evaluate-easy"
                value={ store.evaluate_easy.as_deref().unwrap_or_default().to_string() }
                is_valid={ evaluate_easy_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to evaluate</b>?"
                id="evaluate-hard"
                value={ store.evaluate_hard.as_deref().unwrap_or_default().to_string() }
                is_valid={ evaluate_hard_state }
                variant={ TextAreaVariant::Split }
                onchange={ &store_onchange }/>
            <Range
                label={format!(
                    "How far along (as a percent) do you feel you are toward the final goal? <b>{}</b>",
                    &*range_state)}
                id="completion-percent"
                initial_value={ store.completion_percent.as_deref().unwrap_or("-1").to_string() }
                handle_oninput={range_changed}
                onchange={ &store_onchange }/>
            <TextArea
                label="Does this pace seem likely to succeed?"
                id="pace-succeed"
                value={ store.pace_succeed.as_deref().unwrap_or_default().to_string() }
                is_valid={ pace_succeed_state }
                onchange={ &store_onchange }/>
            <TextArea
                label="Did you meet with your client this week? If not, when was the last time?"
                id="client-meeting"
                value={ store.client_meeting.as_deref().unwrap_or_default().to_string() }
                is_valid={ client_meeting_state }
                onchange={ &store_onchange }/>
            <TextArea
                label="Are there any issues, concern, or comments about the project?"
                id="issues-comments"
                value={ store.issues_comments.as_deref().unwrap_or_default().to_string() }
                onchange={ &store_onchange }/>
            <Button
                variant={ ButtonVariant::Danger }
                label="Submit"
                class="mt-3 col-auto ms-2" />
        </TeamForm>
    }
}
