use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "session")]
// TODO: use local storage instead, so that the data persists across sessions.
// Local storage does not expire, so a custom method of clearing the data is required.
// Especially since data should not persist between sprints.
// One option is to store the date within the store itself.
// And then clear the storage if the date changs.
pub struct IndividualStore {
    pub saturday_hours: Option<i32>,
    pub sunday_hours: Option<i32>,
    pub monday_hours: Option<i32>,
    pub tuesday_hours: Option<i32>,
    pub wednesday_hours: Option<i32>,
    pub thursday_hours: Option<i32>,
    pub friday_hours: Option<i32>,
}
