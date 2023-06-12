use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "session")]
// TODO: The create requirement feature probably does not need to
// use session storage at all. Simple use a use_state for the text
// input / area values instead.
pub struct RequirementsStore {
    pub title: Option<String>,
    pub description: Option<String>,
}
