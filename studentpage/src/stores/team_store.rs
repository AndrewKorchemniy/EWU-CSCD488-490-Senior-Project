use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "session")]
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
