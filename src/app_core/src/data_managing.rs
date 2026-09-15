#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimerState {
    NotStarted = 0,
    Started = 1,
}

pub trait Storage {
    fn init(config_path: &str) -> Self
    where
        Self: Sized;
    fn get_timer_state(&self) -> (TimerState, Option<i32>);
    fn get_projects(&self) -> Vec<String>;
    fn get_tasks_from_project(&self, project_name: &str) -> Vec<String>;
    fn create_project(&self, project_name: &str);
    fn create_task(&self, project_name: &str, task_name: &str);
    fn start_timer_on_task(
        &self,
        project_name: &str,
        task_name: &str,
    ) -> Result<(), Error>;
    fn end_timer_on_task(
        &self,
        entry_to_close: &i32,
        description_input: &str,
    ) -> Result<(), Error>;
    fn link_task_2_task(&self, project_name: &str, task_parent_name: &str, task_child_name: &str);
}

// Compile with implementation selected
#[cfg(feature = "data_sqlite")]
pub mod data_sqlite;

#[cfg(feature = "data_odoo")]
pub mod data_odoo;

#[cfg(all(feature = "data_sqlite", feature = "data_odoo"))]
compile_error!("Features data_sqlite and data_odoo are mutually exclusive");

use anyhow::Error;
// Luego puedes exportar el que esté activo para que `cli` lo consuma sin importar el nombre
#[cfg(feature = "data_sqlite")]
pub use data_sqlite::DataManagerImpl as ActiveDataManager;

#[cfg(feature = "data_odoo")]
pub use data_odoo::OdooStorage as ActiveDataManager;
