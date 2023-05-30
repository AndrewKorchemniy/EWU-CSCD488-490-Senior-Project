use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "session")]
// TODO: use local storage instead, so that the data persists across sessions.
// Local storage does not expire, so a custom method of clearing the data is required.
// Especially since data should not persist between sprints.
// One option is to store the date within the store itself.
pub struct RequirementsStore {
    pub title: Option<String>,
    pub description: Option<String>,
}
