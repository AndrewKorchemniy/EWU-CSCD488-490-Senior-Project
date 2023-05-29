use crate::cli::args::{CreateSprint, SprintCommand, SprintSubcommand};
use crate::repository::db::establish_connection;
use crate::repository::models::{NewSprint, SprintNumDate};
use diesel::prelude::*;

pub fn handle_sprint_command(sprintcmd: SprintCommand) {
    let command = sprintcmd.command;
    match command {
        SprintSubcommand::Create(sprintcmd) => {
            create_sprint(sprintcmd);
        }
        SprintSubcommand::Show => {
            show();
        }
    }
}

pub fn create_sprint(sprintcmd: CreateSprint) {
    println!("creating the sprint: {:?}", sprintcmd);
    use crate::repository::schema::sprint_num_dates::dsl::*;

    let connection = &mut establish_connection();
    let new_sprint = NewSprint {
        sprint_num: sprintcmd.sprint_num,
        sprint_date: &sprintcmd.sprint_date,
    };
    // DATABASE TARGET
    diesel::insert_into(sprint_num_dates)
        .values(&new_sprint)
        .execute(connection)
        .expect("Error saving new sprint");
}

pub fn show() {
    use crate::repository::schema::sprint_num_dates::dsl::*;

    let connection = &mut establish_connection();
    let results = sprint_num_dates
        .load::<SprintNumDate>(connection)
        .expect("Error loading sprint_num_dates");

    println!("Displaying {} sprint_num_dates", results.len());
    for sprint_num_date in results {
        println!(
            "{} {}",
            sprint_num_date.sprint_num, sprint_num_date.sprint_date
        );
    }
}
