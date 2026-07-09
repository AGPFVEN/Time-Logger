use anyhow::Result;
use crate::data_managing::{Storage, TimerState};

pub struct SqliteStorage {
}

impl SqliteStorage {
    // Constructor
    pub fn new() -> Self {
        Self {
        }
    }
}

// Implementación obligatoria del trait Storage
impl Storage for SqliteStorage {
    fn init(&self) -> Result<()> {
        // Aquí irá la lógica para crear la DB si no existe
        Ok(())
    }

    fn get_timer_state(&self) -> TimerState {
        return TimerState::NotStarted;
    }

    fn get_projects(&self) -> Vec<String> {
        Vec::new() // Retorno temporal
    }

    fn get_tasks_from_project(&self, _project_name: &str) -> Result<Vec<String>> {
        Ok(Vec::new()) // Retorno temporal
    }

    fn create_project(&self, _project_name: &str) -> Result<String> {
        Ok(String::new()) // Retorno temporal
    }

    fn create_task(&self, _project_name: &str, _task_name: &str) {
        // Lógica pendiente
    }

    fn start_timer_on_task(&self, _project_name: &str, _task_name: &str) -> Result<()> {
        Ok(())
    }

    fn end_timer_on_task(&self,_input_buffer: &str) -> Result<()> {
        Ok(())
    }
}