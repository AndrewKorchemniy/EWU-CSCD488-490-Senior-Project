// use chrono::prelude::*;
use common::models::status_report;
use diesel::{QueryDsl, RunQueryDsl};
// use std::fmt::Error;
// use std::sync::{Arc, Mutex};
use log::info;

use crate::repository::db::establish_connection;
use crate::repository::models;

pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Database {}
    }
    /*all update section*/

    /// Updates the team report table with the given TeamReport struct.
    pub fn update_team_report(&self, team_in: status_report::TeamReport) {
        // This links to the team_reports model.
        use crate::repository::schema::team_reports::dsl::*;
        // This is the TeamReport model from database\repository\models\
        let team_update = models::TeamReport {
        // Here we take in the data from the team_in
        // and give it to the same variables in TeamReport
        // the reason for the teams.clone is that it will
        // used in the filter and burrowing rules apply.
            teams: team_in.teams.clone(),
            sprint_num: team_in.sprint_num,
            understand_easiest: team_in.understand_easiest,
            understand_hardest: team_in.understand_hardest,
            approach_easiest: team_in.approach_easiest,
            approach_hardest: team_in.approach_hardest,
            solve_easiest: team_in.solve_easiest,
            solve_hardest: team_in.solve_hardest,
            evaluate_easiest: team_in.evaluate_easiest,
            evaluate_hardest: team_in.evaluate_hardest,

            completion: team_in.completion,
            contact: team_in.contact,
            comments: team_in.comments,
        };

        /// Setup connection to the database.
        let connection = &mut establish_connection();

        // DATABASE TARGET
        /// Creates the update_row to be passed to diesel.
        let updated_row = diesel::update(team_reports.find((team_in.teams, team_in.sprint_num)))
            // This is a filter/search for in the database
            .set(&team_update)
            // Once the right user has been selected it pass these values
            .execute(connection)
            // Error handling
            .expect("Error in db_connector update_team_report");
        // this is for debugging features
        info!("Updated {} rows", updated_row);
    }
// TODO Below needs to be debugged, but it whats need to connect to the the database to the backend
    // /// Updates the users table with the given User struct.
    // pub fn update_users(&self, user_in: status_report::User) {
    //     /// This helps pull the user in diesel SQL link
    //     use crate::repository::schema::users::dsl::*;
    //     /// This is the User model from database\repository\models\
    //     let user_update = models::User {
    //         /// here we take in the data from the user_in
    //         /// and give it to the same variables in User
    //         /// the reason for the email.clone is that it will
    //         /// used in the filter and burrowing rules apply
    //         email: user_in.email.clone(),
    //         ouath_id: user_in.ouath_id,
    //         is_teacher: user_in.is_teacher,
    //         is_student: user_in.is_student,
    //         is_admin: user_in.is_admin,
    //         teams: user_in.teams,
    //         class: user_in.class,
    //         first_name: user_in.first_name,
    //         last_name: user_in.last_name,
    //     };
    //
    //     /// makes connection to be used to talk to the database.
    //     let connection = &mut establish_connection();
    //
    //     // DATABASE TARGET
    //     /// this creates the update_row to be passed to diesel
    //     let updated_row = diesel::update(
    //         /// this is a filter/search for in the database
    //         users.find(user_in.email))
    //         ///once the right user has been selected it pass these values
    //         .set(&user_update)
    //         /// where to excute the command
    //         .execute(connection)
    //         ///if the resultif the command fails
    //         .expect("Error in db_connector update_user");
    //     ///this is for debugging features
    //     info!("Updated {} rows", updated_row);
    // }
    //
    // /// Updates the IndividualReport table with the given IndividualReport struct.
    // pub fn update_individual_report(&self, individual_in: status_report::IndividualReport) {
    //     use crate::repository::schema::individual_reports::dsl::*;
    //     let individual_update = models::IndividualReport {
    //         email: individual_in.email.clone(),
    //         sprint_num: individual_in.sprint_num,
    //         monday_time: individual_in.monday_time,
    //         tuesday_time: individual_in.tuesday_time,
    //         wednesday_time: individual_in.wednesday_time,
    //         thursday_time: individual_in.thursday_time,
    //         friday_time: individual_in.friday_time,
    //         saturday_time: individual_in.saturday_time,
    //         sunday_time: individual_in.sunday_time,
    //         discrepancy: individual_in.discrepancy,
    //         request: individual_in.request,
    //     };
    //
    //     let connection = &mut establish_connection();
    //
    //     // DATABASE TARGET
    //     let updated_row = diesel::update(
    //         individual_reports.find((individual_in.email, individual_in.sprint_num)),
    //     )
    //     .set(&individual_update)
    //     .execute(connection)
    //     .expect("Error in db_connector update_individual_report");
    //     info!("Updated {} rows", updated_row);
    // }
    //
    // /// Updates the Requirement table with the given Requirement struct.
    // pub fn update_requirement(&self, requirement_in: status_report::Requirement) {
    //     use crate::repository::schema::requirements::dsl::*;
    //     let requirement_update = models::Requirement {
    //         teams: requirement_in.teams.clone(),
    //         indexs: requirement_in.indexs,
    //         description: requirement_in.description,
    //     };
    //
    //     let connection = &mut establish_connection();
    //
    //     // DATABASE TARGET
    //     let updated_row =
    //         diesel::update(requirements.find((requirement_in.teams, requirement_in.indexs)))
    //             .set(&requirement_update)
    //             .execute(connection)
    //             .expect("Error in db_connector update_requirement");
    //     info!("Updated {} rows", updated_row);
    // }
    //
    // /// Insert the sprint_num_dates table with the given NewSprint struct.
    // pub fn new_sprint_num_date(&self, sprint_new: status_report::NewSprint) {
    //
    //     use crate::repository::schema::sprint_num_dates::dsl::*;
    //
    //     let sprint_create = models::NewSprint
    //     {
    //
    //         sprint_num: sprint_new.sprint_num,
    //         sprint_date: sprint_new.sprint_date,
    //     };
    //
    //     let connection = &mut establish_connection();
    //
    //     // DATABASE TARGET
    //     diesel::insert_into(sprint_num_dates)
    //         .values(&sprint_create)
    //         .execute(connection)
    //         .expect("Error in db_connector new_sprint_num_date");
    // }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
