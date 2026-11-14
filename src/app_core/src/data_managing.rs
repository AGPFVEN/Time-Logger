use anyhow::Error;
use serde::Deserialize;

pub mod data_odoo;
pub mod data_sqlite;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StorageConfig {
    Sqlite {
        database_url: String,
    },
    Odoo {
        url: String,
        db: String,
        username: String,
        token: String,
    },
}

impl StorageConfig {
    pub fn build(self) -> Box<dyn Storage> {
        match self {
            Self::Sqlite { database_url } => {
                Box::new(data_sqlite::SqliteStorage::new(&database_url))
            }
            Self::Odoo {
                url,
                db,
                username,
                token,
            } => Box::new(data_odoo::OdooStorage::new(&url, &db, &username, &token)),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimerState {
    NotStarted = 0,
    Started = 1,
}

pub trait Storage {
    fn get_timer_state(&self) -> (TimerState, Option<i32>);
    fn get_projects(&self) -> Vec<String>;
    fn get_tasks_from_project(&self, project_name: &str) -> Vec<String>;
    fn create_project(&self, project_name: &str);
    fn create_task(&self, project_name: &str, task_name: &str);
    fn start_timer_on_task(&self, project_name: &str, task_name: &str) -> Result<(), Error>;
    fn end_timer_on_task(&self, entry_to_close: &i32, description_input: &str)
    -> Result<(), Error>;
    fn link_task_2_task(&self, project_name: &str, task_parent_name: &str, task_child_name: &str);
}
