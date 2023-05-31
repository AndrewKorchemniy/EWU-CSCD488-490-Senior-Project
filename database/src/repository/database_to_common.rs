// use common::models::status_report::*;
//
//
// impl AsChangeset for TeamReport {
//     type Target = teams_reports::table;
//
//     fn as_changeset(&self) -> Self::Changeset {
//         use teams_reports::dsl::*;
//         let changeset = (
//             teams.eq(&self.teams),
//             sprint_num.eq(&self.sprint_num),
//             understand_easiest.eq(&self.understand_easiest),
//             understand_hardest.eq(&self.understand_hardest),
//             approach_easiest.eq(&self.approach_easiest),
//             approach_hardest.eq(&self.approach_hardest),
//             solve_easiest.eq(&self.solve_easiest),
//             solve_hardest.eq(&self.solve_hardest),
//             evaluate_easiest.eq(&self.evaluate_easiest),
//             evaluate_hardest.eq(&self.evaluate_hardest),
//             completion.eq(&self.completion),
//             contact.eq(&self.contact),
//             comments.eq(&self.comments),
//         );
//         changeset.into_changeset()
//     }
// }
// impl AsChangeset for SprintNumDate {
//     type Target = sprint_num_dates::table;
//
//     fn as_changeset(&self) -> Self::Changeset {
//         use sprint_num_dates::dsl::*;
//         let changeset = (
//             sprint_num.eq(&self.sprint_num),
//             sprint_date.eq(&self.sprint_date),
//         );
//         changeset.into_changeset()
//     }
// }
//
// impl AsChangeset for IndividualReport {
//     type Target = individual_reports::table;
//
//     fn as_changeset(&self) -> Self::Changeset {
//         use individual_reports::dsl::*;
//         let changeset = (
//             email.eq(&self.email),
//             sprint_num.eq(&self.sprint_num),
//             monday_time.eq(&self.monday_time),
//             tuesday_time.eq(&self.tuesday_time),
//             wednesday_time.eq(&self.wednesday_time),
//             thursday_time.eq(&self.thursday_time),
//             friday_time.eq(&self.friday_time),
//             saturday_time.eq(&self.saturday_time),
//             sunday_time.eq(&self.sunday_time),
//             discrepancy.eq(&self.discrepancy),
//             request.eq(&self.request),
//         );
//         changeset.into_changeset()
//     }
// }
//
//
// impl AsChangeset for Requirement {
//     type Target = requirements::table;
//
//     fn as_changeset(&self) -> Self::Changeset {
//         use requirements::dsl::*;
//         let changeset = (
//             teams.eq(&self.teams),
//             indexs.eq(&self.indexs),
//             description.eq(&self.description),
//         );
//         changeset.into_changeset()
//     }
// }
//
//
// impl AsChangeset for TeamActivity {
//     type Target = team_activities::table;
//
//     fn as_changeset(&self) -> Self::Changeset {
//         use team_activities::dsl::*;
//         let changeset = (
//             teams.eq(&self.teams),
//             email.eq(&self.email),
//             sprint_num.eq(&self.sprint_num),
//             activity_index.eq(&self.activity_index),
//             answers.eq(&self.answers),
//         );
//         changeset.into_changeset()
//     }
// }
//
// impl AsChangeset for User {
//     type Target = users::table;
//
//     fn as_changeset(&self) -> Self::Changeset {
//         use users::dsl::*;
//         let changeset = (
//             email.eq(&self.email),
//             ouath_id.eq(&self.ouath_id),
//             is_teacher.eq(&self.is_teacher),
//             is_student.eq(&self.is_student),
//             is_admin.eq(&self.is_admin),
//             teams.eq(&self.teams),
//             class.eq(&self.class),
//             first_name.eq(&self.first_name),
//             last_name.eq(&self.last_name),
//         );
//         changeset.into_changeset()
//     }
// }
//
//
//
//
//
