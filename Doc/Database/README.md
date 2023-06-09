# Database User Guide 
By Nicolas Gainer

[SQL DataBase Design](SQL_Design.md)

to get started we need to install some components in a Ubuntu linux sub-system :

***mysql server***
```
sudo apt-get install libmysqlclient-dev
```
you need to have the libmysqlclient-dev installed before you download the diesel_cli

***diesel*** 
```
cargo install diesel_cli --no-default-features --features mysql
```

the easy way to connect to your deisel to the database is with the .env file which you can create with this line of code in command line(make sure you are in the location you want the file at)which the best location is in the database folder.
***database mysql connection file***
```
echo DATABASE_URL=mysql://username:password@localhost/diesel_demo > .env
```
the db.rs file is the connection file setup which uses this .env file 

this command line will create the database and create an empty migrations directory. (if the database exist and you have the miegrations directory don't worry about this command)
```
diesel setup
```

this will create the migration folder for up and down
which is your create tables and drop tables.
if migration folder exist with up.sql and down.sql dont worry about this line of code.
```
diesel migration generate create_posts
```

when you run this command it will run the up.sql file and any sql will create the tables for the database that you have setup, using sql code. this comannd also creates or updates the scheme.rs
```
diesel migration run
```
this line will fail if you have any tables in the database so you might have to manuilly drop the tables before running.


example of the sql code in the up.sql
```sql
create table users  
(  
email varchar(255) not null primary key,  
ouath_id varchar(255) null,  
is_teacher bool null,  
is_student bool null,  
is_admin bool null,  
teams varchar(50) null,  
class varchar(50) null,  
first_name varchar(30) null,  
last_name varchar(30) null,  
index idx_teams(teams)  
);
```

diesel generated schema.rs code example
```rust
diesel::table! {  
users (email) {  
email -> Varchar,  
ouath_id -> Nullable<Varchar>,  
is_teacher -> Nullable<Bool>,  
is_student -> Nullable<Bool>,  
is_admin -> Nullable<Bool>,  
teams -> Nullable<Varchar>,  
class -> Nullable<Varchar>,  
first_name -> Nullable<Varchar>,  
last_name -> Nullable<Varchar>,  
}  
}
```
If your table has Floats, the diesel type equivalent is f32 and for doubles its f64.(we never got diesel to like doubles)

```rust
#[derive(Debug, Queryable, AsChangeset,Clone)]  
pub struct User {  
pub email: String,  
pub ouath_id: String,  
pub is_teacher: bool,  
pub is_student: bool,  
pub is_admin: bool,  
pub teams: String,  
pub class: String,  
pub first_name: String,  
pub last_name: String,  
}
```
this is in the model.rs which is the model struct used to connect to the schema. (will explain more in the creating models portion.)


the redo command does a rollback by reverting the changes made by the last excuted migration which executes the down migration script associated with the last migration, then the up script reapplies the tables from the up.sql. 
```
diesel migration redo
```
the order of your tables in up matters with the primary key connection points and foregn keys and the down table needs to reverse the up table order.



for models you will need two table, one to create a table for  and a table for updating.

This table is used if you want to update tables or just want to see whats on the table.

#[derive(Debug, Queryable, AsChangeset, Clone)] these are a set of functions added to the struct. 

Debug: trait provides a default string representation of a type, primarily intended for debugging purposes. and used like this "println!("{:?}", my_struct)" when used for a struct

Queryable: trait enables mapping database query results to Rust structs

AsChangeset: provides a convenient way to define changes to be applied to a database record based on the fields of a struct

Clone: allows you to create a copy of a value, providing a convenient way to clone objects
```rust
#[derive(Debug, Queryable, AsChangeset, Clone)]  
pub struct User {  
pub email: String,  
pub ouath_id: String,  
pub is_teacher: bool,  
pub is_student: bool,  
pub is_admin: bool,  
pub teams: String,  
pub class: String,  
pub first_name: String,  
pub last_name: String,  
}
```


This table is used only for when you want to add a new row to your table. 

the "#[diesel(table_name = users)]" line is the connection name to the scheme. these fields are the ones you want to make sure those are the fields needed at creation time.

#[derive(Insertable,Clone)] these are a set of functions added to the struct. 

insertable: is when you want to insert sql

Clone: allows ease of copying one object into another
```rust
#[derive(Insertable,Clone)]  
#[diesel(table_name = users)]  
pub struct NewUser<'a> {  
pub email: &'a str,  
pub ouath_id: &'a str,  
pub is_admin: bool,  
pub first_name: &'a str,  
pub last_name: &'a str,  
}
```




here you have the argument struct for creating the user which is located in the *args.rs file which is the sub command used from the cli User Create.
```rust
#[derive(Debug, Args)]  
pub struct CreateUser {  
pub email: String,  
pub ouath_id: String,  
pub first_name: String,  
pub last_name: String,  
pub teams: String,  
}
```

the same thing for the update struct but has all the fields in the table.
```rust
#[derive(Debug, Args)]  
pub struct UpdateUser {  
pub email: String,  
pub ouath_id: String,  
pub is_teacher: bool,  
pub is_student: bool,  
pub is_admin: bool,  
pub teams: String,  
pub class: String,  
pub first_name: String,  
pub last_name: String,  
}
```


in the *main.rs* file this takes the Command you got from args and sends it to the operation for the spacific table you called.
```rust
EntityType::User(user) => handle_user_command(user)
```

in the match command for *handle_user_command* this will see the Create user command and send it to the create_user function. 
```rust
UserSubcommand::Create(user_cmd) => {  
create_user(user_cmd);  
}
```


from here you make gran the scheme tables made by diesel to connect to the databse. then you make a connection through the *db.rs* file which has your connection point code. from there you make a *new_user* object with the *NewUser* table from models. Then from there you use the **diesel::insert_into** function with the NewUser Object and the connection to the database.
```rust
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
```

the update is very similar but the main differnce is the way you use **diesel::update**. it takes the *scheme table users* with the **find()** function to find the primary key in that table with the command data.  
```rust
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
```


you have to make sure that the **find()** matches the **primary key** for the table you are updating. and if there are 2 or more it has to be added as a tuple. example:  ***(team_report_cmd.teams, team_report_cmd.sprint_num)*** 
```rust
let updated_row =  
diesel::update(team_reports.find((team_report_cmd.teams, team_report_cmd.sprint_num)))  
	.set(&new_team_report)  
	.execute(connection)  
	.expect("Error updating teamReport");
println!("Updated {} rows", updated_row);
```


***USER GUIDE TO DIESEL CLI***

for the most part it is self guiding all you really need to know is to make sure to compile the code and make the call.

make sure to run the build script from the main file so make sure you are in the EWU-CSCD488-490-Senior-Project directory when running this code.
```
helper_scripts/build.sh
```
it will build the exacuables

then run this to excute the help for the cli
```
./target/release/dbcli --help
```

this should be the output.
```
Commands:
  sprint         Create, Show
  team-report    Create, Update
  individual     Create, Update
  requirements   Create, Update
  team-activity  Create, Update
  user           Create, Update
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```


the sites on helping set up the diesel database code:
https://diesel.rs/guides/getting-started
	this site was missing libmysqlclient-dev information but besides that it was a good helper.

https://www.youtube.com/watch?v=tRC4EIKhMzw&ab_channel=CodetotheMoon
	this was the video that helped explain more, but I wish they explained a bit more of each connection piece. though there github repo for the setup they have was beautiful. and is what I followed for the setup on our database on the command line interface. 
	https://github.com/Me163/youtube/tree/main/Rustflix

