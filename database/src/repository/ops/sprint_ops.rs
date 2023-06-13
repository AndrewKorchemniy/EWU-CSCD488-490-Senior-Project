use crate::cli::args::{CreateSprint, SprintCommand, SprintSubcommand};
use crate::repository::db::establish_connection;
use crate::repository::models::{NewSprint, SprintNumDate};
use diesel::prelude::*;

///command got from main and then sends the next command here to the function called.
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

/// takes in the command, targets the table you want to add to it, establishes a connection to the database,
/// makes a object with the create struct, and injects the object into the database
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
/// takes in the command, targets the table you want to view, establishes a connection to the database,
/// prints the targeted table to the command line.
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
