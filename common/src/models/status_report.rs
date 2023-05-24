use crate::schema::*;
use chrono::NaiveDate;


#[derive(Queryable, Debug, AsChangeset)]
pub struct SprintNumDate {
    pub sprint_num: i32,
    pub sprint_date: NaiveDate,
}

#[derive(Insertable)]
// db target must be specified as table_name
#[diesel(table_name = sprint_num_dates)]
pub struct NewSprint<'a> {
    pub sprint_num: i32,
    pub sprint_date: &'a NaiveDate,
}

#[derive(Queryable, Debug, AsChangeset)]
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
}

#[derive(Insertable)]
#[diesel(table_name = team_reports)]
pub struct NewTeamReport<'a> {
    pub teams: &'a str,
    pub sprint_num: i32,
}

#[derive(Queryable, Debug, AsChangeset)]
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
}

#[derive(Insertable)]
#[diesel(table_name = individual_reports)]
pub struct NewIndividualReport<'a> {
    pub email: &'a str,
    pub sprint_num: i32,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Requirement {

    pub teams       : String,
    pub indexs      : i32,
    pub description : String,
}

#[derive(Insertable)]
#[diesel(table_name = requirements)]
pub struct NewRequirement<'a> {
    pub teams   : &'a str,
    pub indexs  : i32,
}

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = team_activities)]
pub struct TeamActivity {
    pub teams           : String,
    pub email           : String,
    pub sprint_num      : i32,
    pub activity_index  : i32,
    pub answers         : String,
}

#[derive(Insertable)]
#[diesel(table_name = team_activities)]
pub struct NewTeamActivity<'a> {
    pub teams           : &'a str,
    pub email           : &'a str,
    pub sprint_num      : i32,
}

#[derive(Debug, Queryable, AsChangeset)]
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

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub ouath_id: &'a str,
    pub is_admin: bool,
    pub first_name: &'a str,
    pub last_name: &'a str,
}


