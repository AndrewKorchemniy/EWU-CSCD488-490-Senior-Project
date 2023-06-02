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
        use crate::repository::schema::team_reports::dsl::*;
        let team_update = models::TeamReport {
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

        let connection = &mut establish_connection();

        // DATABASE TARGET
        let updated_row = diesel::update(team_reports.find((team_in.teams, team_in.sprint_num)))
            .set(&team_update)
            .execute(connection)
            .expect("Error in db_connector update_team_report");
        info!("Updated {} rows", updated_row);
    }

    /// Updates the users table with the given User struct.
    pub fn update_users(&self, user_in: status_report::User) {
        use crate::repository::schema::users::dsl::*;
        let user_update = models::User {
            email: user_in.email.clone(),
            ouath_id: user_in.ouath_id,
            is_teacher: user_in.is_teacher,
            is_student: user_in.is_student,
            is_admin: user_in.is_admin,
            teams: user_in.teams,
            class: user_in.class,
            first_name: user_in.first_name,
            last_name: user_in.last_name,
        };

        let connection = &mut establish_connection();

        // DATABASE TARGET
        let updated_row = diesel::update(users.find(user_in.email))
            .set(&user_update)
            .execute(connection)
            .expect("Error in db_connector update_user");
        info!("Updated {} rows", updated_row);
    }


    /// Updates the IndividualReport table with the given IndividualReport struct.
    pub fn update_individual_report(&self, individual_in: status_report::IndividualReport) {
        use crate::repository::schema::individual_reports::dsl::*;
        let individual_update = models::IndividualReport {
            email: individual_in.email.clone(),
            sprint_num: individual_in.sprint_num,
            monday_time: individual_in.monday_time,
            tuesday_time: individual_in.tuesday_time,
            wednesday_time: individual_in.wednesday_time,
            thursday_time: individual_in.thursday_time,
            friday_time: individual_in.friday_time,
            saturday_time: individual_in.saturday_time,
            sunday_time: individual_in.sunday_time,
            discrepancy: individual_in.discrepancy,
            request: individual_in.request,
        };

        let connection = &mut establish_connection();

        // DATABASE TARGET
        let updated_row = diesel::update(individual_reports.find((individual_in.email, individual_in.sprint_num)))
            .set(&individual_update)
            .execute(connection)
            .expect("Error in db_connector update_individual_report");
        info!("Updated {} rows", updated_row);
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
