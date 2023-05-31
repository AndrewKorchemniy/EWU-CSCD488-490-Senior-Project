use chrono::prelude::*;
use common::models::status_report;
use diesel::{QueryDsl, RunQueryDsl};
use std::fmt::Error;
use std::sync::{Arc, Mutex};

use crate::repository::db::establish_connection;
use crate::repository::models;
use common::models::todo::Todo;

pub struct Database {
    pub sprints: Arc<Mutex<Vec<Todo>>>,
}

impl Database {
    pub fn new() -> Self {
        let todos = Arc::new(Mutex::new(vec![]));
        Database { sprints: todos }
    }

    /*all create section*/
    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let mut todos = self.sprints.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let todo = Todo {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todos.push(todo.clone());
        Ok(todo)
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let todos = self.sprints.lock().unwrap();
        todos.clone()
    }

    pub fn get_todo_by_id(&self, id: &str) -> Option<Todo> {
        let todos = self.sprints.lock().unwrap();
        todos
            .iter()
            .find(|todo| todo.id == Some(id.to_string()))
            .cloned()
    }
    /*all update section*/

    pub fn update_team_report(team_in: status_report::TeamReport) {
        use crate::repository::schema::team_reports::dsl::*;
        let team_update = models::TeamReport {
            teams: team_in.teams.clone(),
            sprint_num: team_in.sprint_num,
            understand_easiest: team_in.understand_easiest,
            understand_hardest: team_in.understand_hardest,
            approach_easiest: team_in.approach_easiest,
            approach_hardest: team_in.approach_hardest,
            solve_easiest: team_in.solve_easiest,
            solve_hardest: team_in.solve_hardest,
            evaluate_easiest: team_in.evaluate_easiest,
            evaluate_hardest: team_in.evaluate_hardest,
            completion: team_in.completion,
            contact: team_in.contact,
            comments: team_in.comments,
        };

        let connection = &mut establish_connection();

        // DATABASE TARGET
        let updated_row = diesel::update(team_reports.find((team_in.teams, team_in.sprint_num)))
            .set(&team_update)
            .execute(connection)
            .expect("Error in db_connector update_team_report");
        println!("Updated {} rows", updated_row);
    }

    pub fn delete_todo_by_id(&self, id: &str) -> Option<Todo> {
        let mut todos = self.sprints.lock().unwrap();
        let index = todos
            .iter()
            .position(|todo| todo.id == Some(id.to_string()))?;
        Some(todos.remove(index))
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
