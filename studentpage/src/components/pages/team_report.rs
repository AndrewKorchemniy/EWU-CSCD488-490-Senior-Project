use web_sys::{HtmlInputElement, HtmlTextAreaElement};
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
    // States for validation.
    let _understand_easy_validation = use_state(|| TextAreaValidation::None);
    let understand_easy_validation = _understand_easy_validation.clone();
    let understand_easy_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _understand_easy_validation.set(value);
    });
    let _understand_hard_validation = use_state(|| TextAreaValidation::None);
    let understand_hard_validation = _understand_hard_validation.clone();
    let understand_hard_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _understand_hard_validation.set(value);
    });
    let _approach_easy_validation = use_state(|| TextAreaValidation::None);
    let approach_easy_validation = _approach_easy_validation.clone();
    let approach_easy_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _approach_easy_validation.set(value);
    });
    let _approach_hard_validation = use_state(|| TextAreaValidation::None);
    let approach_hard_validation = _approach_hard_validation.clone();
    let approach_hard_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _approach_hard_validation.set(value);
    });
    let _solve_easy_validation = use_state(|| TextAreaValidation::None);
    let solve_easy_validation = _solve_easy_validation.clone();
    let solve_easy_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _solve_easy_validation.set(value);
    });
    let _solve_hard_validation = use_state(|| TextAreaValidation::None);
    let solve_hard_validation = _solve_hard_validation.clone();
    let solve_hard_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _solve_hard_validation.set(value);
    });
    let _evaluate_easy_validation = use_state(|| TextAreaValidation::None);
    let evaluate_easy_validation = _evaluate_easy_validation.clone();
    let evaluate_easy_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _evaluate_easy_validation.set(value);
    });
    let _evaluate_hard_validation = use_state(|| TextAreaValidation::None);
    let evaluate_hard_validation = _evaluate_hard_validation.clone();
    let evaluate_hard_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _evaluate_hard_validation.set(value);
    });
    let _pace_succeed_validation = use_state(|| TextAreaValidation::None);
    let pace_succeed_validation = _pace_succeed_validation.clone();
    let pace_succeed_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _pace_succeed_validation.set(value);
    });
    let _client_meeting_validation = use_state(|| TextAreaValidation::None);
    let client_meeting_validation = _client_meeting_validation.clone();
    let client_meeting_validation_changes = Callback::from(move |value: TextAreaValidation| {
        _client_meeting_validation.set(value);
    });
    let _issues_comments_concerns_validation = use_state(|| TextAreaValidation::None);
    let issues_comments_concerns_validation = _issues_comments_concerns_validation.clone();
    let issues_comments_concerns_validation_changes =
        Callback::from(move |value: TextAreaValidation| {
            _issues_comments_concerns_validation.set(value);
        });

    // Callbacks for updating the store.
    let (store, dispatch) = use_store::<TeamStore>();

    let onchange_understand_easy = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.understand_easy = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_understand_hard = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.understand_hard = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_approach_easy = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.approach_easy = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_approach_hard = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.approach_hard = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_solve_easy = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.solve_easy = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_solve_hard = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.solve_hard = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_evaluate_easy = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.evaluate_easy = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_evaluate_hard = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.evaluate_hard = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_completion_percent = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlInputElement>().value();
        store.completion_percent = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_pace_succeed = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.pace_succeed = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_client_meeting = dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.client_meeting = if value.is_empty() { None } else { Some(value) };
    });
    let onchange_issues_comments_concerns =
        dispatch.reduce_mut_callback_with(|store, event: Event| {
        let value: String = event.target_unchecked_into::<HtmlTextAreaElement>().value();
        store.issues_comments_concerns = if value.is_empty() { None } else { Some(value) };
    });

    // Client-side validation.
    let validate = move |store: &mut TeamStore| {
        let mut valid = true;
        match store.understand_easy {
            Some(_) => understand_easy_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                understand_easy_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.understand_hard {
            Some(_) => understand_hard_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                understand_hard_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.approach_easy {
            Some(_) => approach_easy_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                approach_easy_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.approach_hard {
            Some(_) => approach_hard_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                approach_hard_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.solve_easy {
            Some(_) => solve_easy_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                solve_easy_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.solve_hard {
            Some(_) => solve_hard_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                solve_hard_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.evaluate_easy {
            Some(_) => evaluate_easy_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                evaluate_easy_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.evaluate_hard {
            Some(_) => evaluate_hard_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                evaluate_hard_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.pace_succeed {
            Some(_) => pace_succeed_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                pace_succeed_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.client_meeting {
            Some(_) => client_meeting_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                client_meeting_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        match store.issues_comments_concerns {
            Some(_) => issues_comments_concerns_validation_changes.emit(TextAreaValidation::Valid),
            None => {
                issues_comments_concerns_validation_changes.emit(TextAreaValidation::Invalid);
                valid = false;
            }
        }
        valid
    };

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

    // Callback for submitting the form. Triggers client-side validation.
    let onsubmit = dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
        event.prevent_default();
        validate(store);
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
                validation={ understand_easy_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_understand_easy }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to understand</b>?"
                id="understand-hard"
                value={ store.understand_hard.as_deref().unwrap_or_default().to_string() }
                validation={ understand_hard_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_understand_hard }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to approach</b>?"
                id="approach-easy"
                value={ store.approach_easy.as_deref().unwrap_or_default().to_string() }
                validation={ approach_easy_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_approach_easy }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to approach</b>?"
                id="approach-hard"
                value={ store.approach_hard.as_deref().unwrap_or_default().to_string() }
                validation={ approach_hard_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_approach_hard }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to solve</b>?"
                id="solve-easy"
                value={ store.solve_easy.as_deref().unwrap_or_default().to_string() }
                validation={ solve_easy_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_solve_easy }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to solve</b>?"
                id="solve-hard"
                value={ store.solve_hard.as_deref().unwrap_or_default().to_string() }
                validation={ solve_hard_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_solve_hard }/>
            <TextArea
                label="Which aspects of the current work are the <b>easiest to evaluate</b>?"
                id="evaluate-easy"
                value={ store.evaluate_easy.as_deref().unwrap_or_default().to_string() }
                validation={ evaluate_easy_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_evaluate_easy }/>
            <TextArea
                label="Which aspects of the current work are the <b>hardest to evaluate</b>?"
                id="evaluate-hard"
                value={ store.evaluate_hard.as_deref().unwrap_or_default().to_string() }
                validation={ evaluate_hard_validation }
                variant={ TextAreaVariant::Split }
                onchange={ onchange_evaluate_hard }/>
            <Range
                label={format!(
                    "How far along (as a percent) do you feel you are toward the final goal? <b>{}</b>",
                    &*range_state)}
                id="completion-percent"
                initial_value={ store.completion_percent.as_deref().unwrap_or("-1").to_string() }
                handle_oninput={range_changed}
                onchange={ onchange_completion_percent }/>
            <TextArea
                label="Does this pace seem likely to succeed?"
                id="pace-succeed"
                value={ store.pace_succeed.as_deref().unwrap_or_default().to_string() }
                validation={ pace_succeed_validation }
                variant={ TextAreaVariant::Narrow }
                onchange={ onchange_pace_succeed }/>
            <TextArea
                label="Did you meet with your client this week? If not, when was the last time?"
                id="client-meeting"
                value={ store.client_meeting.as_deref().unwrap_or_default().to_string() }
                validation={ client_meeting_validation }
                variant={ TextAreaVariant::Narrow }
                onchange={ onchange_client_meeting }/>
            <TextArea
                label="Are there any issues, concern, or comments about the project?"
                id="issues-concerns-comments"
                value={ store.issues_comments_concerns.as_deref().unwrap_or_default().to_string() }
                validation={ issues_comments_concerns_validation }
                variant={ TextAreaVariant::Narrow }
                onchange={ onchange_issues_comments_concerns }/>
            <Button
                variant={ ButtonVariant::Danger }
                label="Submit"
                class="mt-2 col-auto ms-2" />
        </TeamForm>
    }
}
