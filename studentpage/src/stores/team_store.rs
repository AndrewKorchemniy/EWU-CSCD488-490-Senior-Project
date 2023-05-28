use crate::components::text_area::TextAreaValidation;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "session")]
// TODO: use local storage instead, so that the data persists across sessions.
// Local storage does not expire, so a custom method of clearing the data will be needed.
// Especially since data should not persist between sprints.
// One option is to store the data within the store itself.
pub struct TeamStore {
    pub understand_easy: Option<String>,
    pub understand_hard: Option<String>,
    pub approach_easy: Option<String>,
    pub approach_hard: Option<String>,
    pub solve_easy: Option<String>,
    pub solve_hard: Option<String>,
    pub evaluate_easy: Option<String>,
    pub evaluate_hard: Option<String>,
    pub completion_percent: Option<String>,
    pub pace_succeed: Option<String>,
    pub client_meeting: Option<String>,
    pub issues_comments: Option<String>,
}

/// Perform validation on the value and emit the result to the state.
/// Returns true if the value is valid, false otherwise.
pub fn validate(text: &Option<String>, state: &Callback<TextAreaValidation>) -> bool {
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
}
