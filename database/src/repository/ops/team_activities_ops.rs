use crate::cli::args::{
    CreateTeamActivity, TeamActivityCommand, TeamActivitySubcommand, UpdateTeamActivity,
};
use crate::repository::db::establish_connection;
use crate::repository::models::{NewTeamActivity, TeamActivity};
use diesel::prelude::*;

///command got from main and then sends the next command here to the function called.
pub fn handle_team_activity_command(team_activity_cmd: TeamActivityCommand) {
    let command = team_activity_cmd.command;
    match command {
        TeamActivitySubcommand::Create(team_activity_cmd) => {
            create_team_activity(team_activity_cmd);
        }
        TeamActivitySubcommand::Update(team_activity_cmd) => {
            update_team_activity(team_activity_cmd);
        }
    }
}

/// takes in the command, targets the table you want to add to it, establishes a connection to the database,
/// makes a object with the create struct, and injects the object into the database
pub fn create_team_activity(team_activity_cmd: CreateTeamActivity) {
    println!("creating the team activity: {:?}", team_activity_cmd);
    use crate::repository::schema::team_activities::dsl::*;

    let connection = &mut establish_connection();
    let new_team_activity = NewTeamActivity {
        teams: &team_activity_cmd.teams,
        email: &team_activity_cmd.email,
        sprint_num: team_activity_cmd.sprint_num,
    };
    // DATABASE TARGET
    diesel::insert_into(team_activities)
        .values(&new_team_activity)
        .execute(connection)
        .expect("Error saving new team activity");
}
/// takes in the command, targets the table you want to add rows to, establishes a connection to the database,
/// makes a object with the update struct, and adds the object to the table.
pub fn update_team_activity(team_activity_cmd: UpdateTeamActivity) {
    println!("updating the requirement: {:?}", team_activity_cmd);
    use crate::repository::schema::team_activities::dsl::*;

    let connection = &mut establish_connection();
    let new_team_activity = TeamActivity {
        teams: team_activity_cmd.teams.clone(),
        email: team_activity_cmd.email.clone(),
        sprint_num: team_activity_cmd.sprint_num,
        activity_index: team_activity_cmd.activity_index,
        answers: team_activity_cmd.answers,
    };

    let updated_row =
        diesel::update(team_activities.find((team_activity_cmd.teams, team_activity_cmd.email)))
            .set(&new_team_activity)
            .execute(connection)
            .expect("Error updating requirement");
    println!("Updated {} rows", updated_row);
}
