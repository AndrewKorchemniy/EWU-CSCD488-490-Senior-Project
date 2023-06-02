use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DatabaseArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    ///Create, Show
    Sprint(SprintCommand),
    ///Create, Update
    TeamReport(TeamReportCommand),
    ///Create, Update
    Individual(IndividualReportCommand),
    ///Create, Update
    Requirements(RequirementCommand),
    ///Create, Update
    TeamActivity(TeamActivityCommand),
    ///Create, Update
    User(UserCommand),
}

//start Sprint commands
#[derive(Debug, Args)]
pub struct SprintCommand {
    #[clap(subcommand)]
    pub command: SprintSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SprintSubcommand {
    /// Create a new sprint
    Create(CreateSprint),

    /// Show all Sprints
    Show,
}

#[derive(Debug, Args)]
pub struct CreateSprint {
    /// The number of the sprint to create
    pub sprint_num: i32,

    /// The sprint date to create
    pub sprint_date: NaiveDate,
}

//start TeamReport commands
#[derive(Debug, Args)]
pub struct TeamReportCommand {
    #[clap(subcommand)]
    pub command: TeamReportSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TeamReportSubcommand {
    /// Create a new TeamReports
    Create(CreateTeamReport),

    /// Update an existing TeamReports
    Update(UpdateTeamReport),
}

#[derive(Debug, Args)]
pub struct CreateTeamReport {
    pub teams: String,
    pub sprint_num: i32,
}

#[derive(Debug, Args)]
pub struct UpdateTeamReport {
    pub teams: String,
    pub sprint_num: i32,
    pub understand_easiest: String,
    pub understand_hardest: String,
    pub approach_easiest: String,
    pub approach_hardest: String,
    pub solve_easiest: String,
    pub solve_hardest: String,
    pub evaluate_easiest: String,
    pub evaluate_hardest: String,
    pub completion: i32,
    pub contact: String,
    pub comments: String,
}

//start IndividualReport commands
#[derive(Debug, Args)]
pub struct IndividualReportCommand {
    #[clap(subcommand)]
    pub command: IndividualReportSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum IndividualReportSubcommand {
    /// Create a new sprint
    Create(CreateIndividualReport),

    /// Update an existing sprint
    Update(UpdateIndividualReport),
}

#[derive(Debug, Args)]
pub struct CreateIndividualReport {
    pub email: String,
    pub sprint_num: i32,
}

#[derive(Debug, Args)]
pub struct UpdateIndividualReport {
    pub email: String,
    pub sprint_num: i32,
    pub monday_time: i32,
    pub tuesday_time: i32,
    pub wednesday_time: i32,
    pub thursday_time: i32,
    pub friday_time: i32,
    pub saturday_time: i32,
    pub sunday_time: i32,
    pub discrepancy: String,
    pub request: String,
}

// commands args for requirements
#[derive(Debug, Args)]
pub struct RequirementCommand {
    #[clap(subcommand)]
    pub command: RequirementSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum RequirementSubcommand {
    /// Create a new requirement
    Create(CreateRequirement),

    /// Update an existing requirement
    Update(UpdateRequirement),
}

#[derive(Debug, Args)]
pub struct CreateRequirement {
    pub teams: String,
    pub indexs: i32,
}

#[derive(Debug, Args)]
pub struct UpdateRequirement {
    pub teams: String,
    pub indexs: i32,
    pub description: String,
}
// commands args for team activities
#[derive(Debug, Args)]
pub struct TeamActivityCommand {
    #[clap(subcommand)]
    pub command: TeamActivitySubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TeamActivitySubcommand {
    /// Create a new team activity
    Create(CreateTeamActivity),

    /// Update an existing team activity
    Update(UpdateTeamActivity),
}

#[derive(Debug, Args)]
pub struct CreateTeamActivity {
    pub teams: String,
    pub email: String,
    pub sprint_num: i32,
}

#[derive(Debug, Args)]
pub struct UpdateTeamActivity {
    pub teams: String,
    pub email: String,
    pub sprint_num: i32,
    pub activity_index: i32,
    pub answers: String,
}

// commands args for Users
#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new team activity
    Create(CreateUser),

    /// Update an existing team activity
    Update(UpdateUser),
}

#[derive(Debug, Args)]
pub struct CreateUser {
    pub email: String,
    pub ouath_id: String,
    pub first_name: String,
    pub last_name: String,
    pub teams: String,
}

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
