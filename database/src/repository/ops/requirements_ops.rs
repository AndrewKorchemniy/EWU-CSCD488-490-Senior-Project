use crate::cli::args::{
    CreateRequirement, RequirementCommand, RequirementSubcommand, UpdateRequirement,
};
use crate::repository::db::establish_connection;
use crate::repository::models::{NewRequirement, Requirement};
use diesel::prelude::*;

///command got from main and then sends the next command here to the function called.
pub fn handle_requirement_command(requirement_cmd: RequirementCommand) {
    let command = requirement_cmd.command;
    match command {
        RequirementSubcommand::Create(requirement_cmd) => {
            create_requirement(requirement_cmd);
        }
        RequirementSubcommand::Update(requirement_cmd) => {
            update_requirement(requirement_cmd);
        }
    }
}

/// takes in the command, targets the table you want to add to it, establishes a connection to the database,
/// makes a object with the create struct, and injects the object into the database
pub fn create_requirement(requirement_cmd: CreateRequirement) {
    println!("creating the requirement: {:?}", requirement_cmd);
    use crate::repository::schema::requirements::dsl::*;

    let connection = &mut establish_connection();
    let new_requirement = NewRequirement {
        teams: &requirement_cmd.teams,
        indexs: requirement_cmd.indexs,
    };
    // DATABASE TARGET
    diesel::insert_into(requirements)
        .values(&new_requirement)
        .execute(connection)
        .expect("Error saving new requirement");
}

/// takes in the command, targets the table you want to add rows to, establishes a connection to the database,
/// makes a object with the update struct, and adds the object to the table.
pub fn update_requirement(requirement_cmd: UpdateRequirement) {
    println!("updating the requirement: {:?}", requirement_cmd);
    use crate::repository::schema::requirements::dsl::*;

    let connection = &mut establish_connection();
    let new_requirement = Requirement {
        teams: requirement_cmd.teams.clone(),
        indexs: requirement_cmd.indexs.clone(),
        description: requirement_cmd.description,
    };

    let updated_row =
        diesel::update(requirements.find((requirement_cmd.indexs, requirement_cmd.teams)))
            .set(&new_requirement)
            .execute(connection)
            .expect("Error updating requirement");
    println!("Updated {} rows", updated_row);
}
