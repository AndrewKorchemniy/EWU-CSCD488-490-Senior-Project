use crate::cli::args::{
    CreateRequirement, RequirementCommand, RequirementSubcommand, UpdateRequirement,
};
use crate::repository::db::establish_connection;
use crate::repository::models::{NewRequirement, Requirement};
use diesel::prelude::*;

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
