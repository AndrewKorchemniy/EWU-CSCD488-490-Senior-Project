// @generated automatically by Diesel CLI.

diesel::table! {
    individual_reports (email, sprint_num) {
        email -> Varchar,
        sprint_num -> Integer,
        monday_time -> Nullable<Integer>,
        tuesday_time -> Nullable<Integer>,
        wednesday_time -> Nullable<Integer>,
        thursday_time -> Nullable<Integer>,
        friday_time -> Nullable<Integer>,
        saturday_time -> Nullable<Integer>,
        sunday_time -> Nullable<Integer>,
        discrepancy -> Nullable<Varchar>,
        request -> Nullable<Varchar>,
    }
}

diesel::table! {
    requirements (indexs, teams) {
        teams -> Varchar,
        indexs -> Integer,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    sprint_num_dates (sprint_num) {
        sprint_num -> Integer,
        sprint_date -> Date,
    }
}

diesel::table! {
    team_activities (teams, email) {
        teams -> Varchar,
        email -> Varchar,
        sprint_num -> Nullable<Integer>,
        activity_index -> Nullable<Integer>,
        answers -> Nullable<Varchar>,
    }
}

diesel::table! {
    team_reports (teams, sprint_num) {
        teams -> Varchar,
        sprint_num -> Integer,
        understand_easiest -> Nullable<Varchar>,
        understand_hardest -> Nullable<Varchar>,
        approach_easiest -> Nullable<Varchar>,
        approach_hardest -> Nullable<Varchar>,
        solve_easiest -> Nullable<Varchar>,
        solve_hardest -> Nullable<Varchar>,
        evaluate_easiest -> Nullable<Varchar>,
        evaluate_hardest -> Nullable<Varchar>,
        completion -> Nullable<Integer>,
        contact -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
    }
}

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

diesel::joinable!(individual_reports -> sprint_num_dates (sprint_num));
diesel::joinable!(individual_reports -> users (email));
diesel::joinable!(team_activities -> sprint_num_dates (sprint_num));
diesel::joinable!(team_reports -> sprint_num_dates (sprint_num));

diesel::allow_tables_to_appear_in_same_query!(
    individual_reports,
    requirements,
    sprint_num_dates,
    team_activities,
    team_reports,
    users,
);
