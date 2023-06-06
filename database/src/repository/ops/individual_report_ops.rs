use crate::cli::args::{
    CreateIndividualReport, IndividualReportCommand, IndividualReportSubcommand,
    UpdateIndividualReport,
};
use crate::repository::db::establish_connection;
use crate::repository::models::{IndividualReport, NewIndividualReport};
use diesel::prelude::*;

///command got from main and then sends the next command here to the function called.
pub fn handle_individual_report_command(individual_report_cmd: IndividualReportCommand) {
    let command = individual_report_cmd.command;
    match command {
        IndividualReportSubcommand::Create(individual_report_cmd) => {
            create_individual_report(individual_report_cmd);
        }
        IndividualReportSubcommand::Update(individual_report_cmd) => {
            update_individual_report(individual_report_cmd);
        }
    }
}
/// takes in the command, targets the table you want to add to it, establishes a connection to the database,
/// makes a object with the create struct, and injects the object into the database
pub fn create_individual_report(individual_report_cmd: CreateIndividualReport) {
    println!(
        "creating the Individual Report: {:?}",
        individual_report_cmd
    );
    use crate::repository::schema::individual_reports::dsl::*;

    let connection = &mut establish_connection();
    let new_individual_report = NewIndividualReport {
        email: &individual_report_cmd.email,
        sprint_num: individual_report_cmd.sprint_num,
    };
    // DATABASE TARGET
    diesel::insert_into(individual_reports)
        .values(&new_individual_report)
        .execute(connection)
        .expect("Error saving new individualReport");
}

/// takes in the command, targets the table you want to add rows to, establishes a connection to the database,
/// makes a object with the update struct, and adds the object to the table.
pub fn update_individual_report(individual_report_cmd: UpdateIndividualReport) {
    println!("updating the individualReport: {:?}", individual_report_cmd);
    use crate::repository::schema::individual_reports::dsl::*;

    let connection = &mut establish_connection();
    let new_individual_report = IndividualReport {
        email: individual_report_cmd.email.clone(),
        sprint_num: individual_report_cmd.sprint_num,
        monday_time: individual_report_cmd.monday_time,
        tuesday_time: individual_report_cmd.tuesday_time,
        wednesday_time: individual_report_cmd.wednesday_time,
        thursday_time: individual_report_cmd.thursday_time,
        friday_time: individual_report_cmd.friday_time,
        saturday_time: individual_report_cmd.saturday_time,
        sunday_time: individual_report_cmd.sunday_time,
        discrepancy: individual_report_cmd.discrepancy,
        request: individual_report_cmd.request,
    };

    let updated_row = diesel::update(individual_reports.find((
        individual_report_cmd.email,
        individual_report_cmd.sprint_num,
    )))
    .set(&new_individual_report)
    .execute(connection)
    .expect("Error updating individualReport");

    println!("Updated {} rows", updated_row);
}
