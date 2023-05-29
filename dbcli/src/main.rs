#[macro_use]
extern crate diesel;
extern crate dotenv;

use database::repository::ops;
use database::cli::args;
use database::repository::db;
use database::repository::models;
use database::repository::schema;



use ops::individual_report_ops::handle_individual_report_command;
use ops::sprint_ops::handle_sprint_command;
use ops::team_report_ops::handle_team_report_command;
use ops::requirements_ops::handle_requirement_command;
use ops::team_activities_ops::handle_team_activity_command;
use ops::user_ops::handle_user_command;

use args::DatabaseArgs;
use args::EntityType;
use clap::Parser;

fn main() {
    let args = DatabaseArgs::parse();

    match args.entity_type {
        // if "this" matches "that"
        EntityType::Sprint(sprint) => handle_sprint_command(sprint),
        EntityType::TeamReport(team_report) => handle_team_report_command(team_report),
        EntityType::Individual(individual_report) => handle_individual_report_command(individual_report) ,
        EntityType::Requirements(requirement) => handle_requirement_command(requirement),
        EntityType::TeamActivity(team_activity ) => handle_team_activity_command(team_activity),
        EntityType::User(user) => handle_user_command(user),
    }
}
