pub mod text_storage;
pub mod sqlite_storage;

use anyhow::Result;

pub trait Storage {
    fn init(&self) -> Result<()>;
    fn get_projects(&self) -> Vec<String>;
    fn get_tasks_from_project(&self, project_name: &str) -> Result<Vec<String>>;
    fn create_project(&self, project_name: &str) -> Result<String>;
    fn create_task(&self, project_name: &str, task_name: &str);
    fn start_timer_on_task(&self, project_name: &str, task_name: &str) -> Result<()>;
    fn end_timer_on_task(&self, project_name: &str, task_name: &str, input_buffer: &str) -> Result<()>;
}
