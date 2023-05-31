use chrono::NaiveDate;

#[derive(Clone)]
pub struct SprintNumDate {
    pub sprint_num: i32,
    pub sprint_date: NaiveDate,
}
#[derive(Clone)]
pub struct NewSprint<'a> {
    pub sprint_num: i32,
    pub sprint_date: &'a NaiveDate,
}

#[derive(Clone)]
pub struct TeamReport {
    pub teams: String,
    pub sprint_num: i32,
    pub understand_easiest: String,
    pub understand_hardest: String,
    pub approach_easiest: String,
    pub approach_hardest: String,
    pub solve_easiest: String,
    pub solve_hardest: String,
    pub evaluate_easiest: String,
    pub evaluate_hardest: String,
    pub completion: i32,
    pub contact: String,
    pub comments: String,
    // TODO: pub is_completed: bool,
}
#[derive(Clone)]
pub struct NewTeamReport<'a> {
    pub teams: &'a str,
    pub sprint_num: i32,
}

#[derive(Clone)]
pub struct IndividualReport {
    pub email: String,
    pub sprint_num: i32,
    pub monday_time: i32,
    pub tuesday_time: i32,
    pub wednesday_time: i32,
    pub thursday_time: i32,
    pub friday_time: i32,
    pub saturday_time: i32,
    pub sunday_time: i32,
    pub discrepancy: String,
    pub request: String,
    // TODO: pub is_completed: bool,
}

#[derive(Clone)]
pub struct NewIndividualReport<'a> {
    pub email: &'a str,
    pub sprint_num: i32,
}

#[derive(Clone)]
pub struct Requirement {
    pub teams: String,
    pub indexs: i32,
    pub description: String,
}

#[derive(Clone)]
pub struct NewRequirement<'a> {
    pub teams: &'a str,
    pub indexs: i32,
}

#[derive(Clone)]
pub struct TeamActivity {
    pub teams: String,
    pub email: String,
    pub sprint_num: i32,
    pub activity_index: i32,
    pub answers: String,
}

#[derive(Clone)]
pub struct NewTeamActivity<'a> {
    pub teams: &'a str,
    pub email: &'a str,
    pub sprint_num: i32,
}

#[derive(Clone)]
pub struct User {
    pub email: String,
    pub ouath_id: String,
    pub is_teacher: bool,
    pub is_student: bool,
    pub is_admin: bool,
    pub teams: String,
    pub class: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub ouath_id: &'a str,
    pub is_admin: bool,
    pub first_name: &'a str,
    pub last_name: &'a str,
}
