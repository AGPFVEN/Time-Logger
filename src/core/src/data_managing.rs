pub mod text_storage;
pub mod sqlite_storage;

use anyhow::Result;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)] 
pub enum TimerState {
    NotStarted = 0,
    Started = 1,
}

pub trait Storage {
    fn init(&self) -> Result<()>;
    fn get_timer_state(&self) -> TimerState;
    fn get_projects(&self) -> Vec<String>;
    fn get_tasks_from_project(&self, project_name: &str) -> Result<Vec<String>>;
    fn create_project(&self, project_name: &str) -> Result<String>;
    fn create_task(&self, project_name: &str, task_name: &str);
    fn start_timer_on_task(&self, project_name: &str, task_name: &str) -> Result<()>;
    fn end_timer_on_task(&self, input_buffer: &str) -> Result<()>;
}
