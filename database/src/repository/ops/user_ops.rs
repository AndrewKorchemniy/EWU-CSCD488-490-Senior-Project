use crate::cli::args::{CreateUser, UpdateUser, UserCommand, UserSubcommand};
use crate::repository::db::establish_connection;
use crate::repository::models::{NewUser, User};
use diesel::prelude::*;

///command got from main and then sends the next command here to the function called.
pub fn handle_user_command(user_cmd: UserCommand) {
    let command = user_cmd.command;
    match command {
        UserSubcommand::Create(user_cmd) => {
            create_user(user_cmd);
        }
        UserSubcommand::Update(user_cmd) => {
            update_user(user_cmd);
        }
    }
}
/// takes in the command, targets the table you want to add to it, establishes a connection to the database,
/// makes a object with the create struct, and injects the object into the database
pub fn create_user(user_cmd: CreateUser) {
    println!("creating thee user: {:?}", user_cmd);
    use crate::repository::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let new_user = NewUser {
        email: &user_cmd.email,
        ouath_id: &user_cmd.ouath_id,
        first_name: &user_cmd.first_name,
        last_name: &user_cmd.last_name,
        teams: &user_cmd.teams,
    };
    // DATABASE TARGET
    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");
}

/// takes in the command, targets the table you want to add rows to, establishes a connection to the database,
/// makes a object with the update struct, and adds the object to the table.
pub fn update_user(user_cmd: UpdateUser) {
    println!("updating the requirement: {:?}", user_cmd);
    use crate::repository::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let new_user = User {
        email: user_cmd.email.clone(),
        ouath_id: user_cmd.ouath_id,
        is_teacher: user_cmd.is_teacher,
        is_student: user_cmd.is_student,
        is_admin: user_cmd.is_admin,
        teams: user_cmd.teams,
        class: user_cmd.class,
        first_name: user_cmd.first_name,
        last_name: user_cmd.last_name,
    };

    let updated_row = diesel::update(users.find(user_cmd.email))
        .set(&new_user)
        .execute(connection)
        .expect("Error updating requirement");
    println!("Updated {} rows", updated_row);
}
