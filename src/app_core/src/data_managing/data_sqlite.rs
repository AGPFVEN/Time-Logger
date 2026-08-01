use chrono::Local;
use diesel::prelude::*;
use std::sync::{Mutex, MutexGuard};

mod db;
mod models;
mod queries;
mod schema;

use super::{Storage, TimerState};

pub struct SqliteStorage {
    connection: Mutex<SqliteConnection>,
}

impl SqliteStorage {
    fn new(db_path: &str) -> Self {
        Self {
            connection: Mutex::new(db::establish_connection(db_path)),
        }
    }

    fn lock_db_mutex(&self) -> MutexGuard<'_, SqliteConnection> {
        return self
            .connection
            .lock()
            .expect("Failed to acquire database lock");
    }
}

impl Storage for SqliteStorage {
    fn init(db_path: &str) -> Self {
        Self::new(db_path)
    }

    fn get_timer_state(&self) -> (TimerState, Option<i32>) {
        match queries::time_entries::find_incomplete(&mut self.lock_db_mutex()) {
            Some(entry) => (TimerState::Started, Some(entry.id)),
            None => (TimerState::NotStarted, None),
        }
    }

    fn get_projects(&self) -> Vec<String> {
        queries::project::get_all_projects(&mut self.lock_db_mutex())
            .into_iter()
            .collect()
    }

    fn create_project(&self, project_name: &str) {
        queries::project::create_project(
            &mut self.lock_db_mutex(),
            project_name,
            None,
            None,
            None,
            "Placeholder",
            "Placeholder",
            None,
        );
    }

    fn get_tasks_from_project(&self, project_name: &str) -> Vec<String> {
        queries::task::get_tasks_from_project(&mut self.lock_db_mutex(), project_name)
    }

    fn create_task(&self, project_name: &str, task_name: &str) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        queries::task::create_task(
            &mut self.lock_db_mutex(),
            task_name,
            None,
            None,
            None,
            None,
            None,
            &now,
            &now,
            Some(0),
        );
        queries::project::link_task_to_project(&mut self.lock_db_mutex(), project_name, task_name);
    }

    fn start_timer_on_task(
        &self,
        project_name: &str,
        task_name: &str,
    ) -> Result<(), diesel::result::Error> {
        // This constant is used for the mutex to be closed as less time as possible
        let start_time_formatted = &Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        return queries::time_entries::create_time_entry(
            &mut self.lock_db_mutex(),
            project_name,
            task_name,
            &start_time_formatted,
        );
    }

    fn end_timer_on_task(
        &self,
        entry_to_close_id: &i32,
        description_input: &str,
    ) -> Result<(), diesel::result::Error> {
        // This constant is used for the mutex to be closed as less time as possible
        let end_time_formatted = &Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        queries::time_entries::close_time_entry(
            &mut self.lock_db_mutex(),
            entry_to_close_id,
            end_time_formatted,
            description_input,
        )
    }

    fn link_task_2_task(&self, project_name: &str, task_parent_name: &str, task_child_name: &str) {
        queries::task::link_task_2_task(
            &mut self.lock_db_mutex(),
            project_name,
            task_parent_name,
            task_child_name,
        );
    }
}
