pub mod data_sqlite;

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
    ) -> Result<(), diesel::result::Error>;
    fn end_timer_on_task(
        &self,
        entry_to_close: &i32,
        description_input: &str,
    ) -> Result<(), diesel::result::Error>;
    fn link_task_2_task(&self, project_name: &str, task_parent_name: &str, task_child_name: &str);
}
