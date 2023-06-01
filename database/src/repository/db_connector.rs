use chrono::prelude::*;
use common::models::status_report;
use diesel::{QueryDsl, RunQueryDsl};
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use log::info;

use crate::repository::db::establish_connection;
use crate::repository::models;
use common::models::todo::Todo;

pub struct Database {
}

impl Database {
    pub fn new() -> Self {
        Database { }
    }
    /*all update section*/

    pub fn update_team_report(team_in: status_report::TeamReport) {
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
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
