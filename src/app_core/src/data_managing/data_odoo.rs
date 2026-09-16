use anyhow::Error;
use chrono::Local;
use odoo_api::{
    OdooClient,
    client::{Authed, ReqwestBlocking},
    jmap, jvec,
    service::object::ExecuteKwResponse,
};
use serde_json::from_str;
use std::{cell::RefCell, fs};
use std::{path::Path, result::Result::Ok};

use super::{Storage, TimerState};

pub struct OdooStorage {
    odoo_client: RefCell<OdooClient<Authed, ReqwestBlocking>>,
    projects: Option<ExecuteKwResponse>,
}

// TODO: No usar magic strings para los datos como la db, user y password
impl OdooStorage {
    pub fn new() -> Self {
        let client = match OdooClient::new_reqwest_blocking("http://100.74.16.39:10018") {
            Ok(res) => res,
            Err(err) => {
                panic!("An error ocurred when sending request to odoo: {}", err);
            }
        };

        let authenticated_client = match client.authenticate("testing", "admin", "a") {
            Ok(res) => res,
            Err(error) => {
                panic!("An error ocurred when authenticating to odoo: {}", error);
            }
        };

        return Self {
            odoo_client: RefCell::new(authenticated_client),
            projects: None,
        };
    }
}

impl Storage for OdooStorage {
    fn init(db_path: &str) -> Self {
        Self::new()
    }

    fn get_timer_state(&self) -> (TimerState, Option<i32>) {
        let nombre_archivo = "cualquier_nombre.txt";

        if Path::new(nombre_archivo).exists() {
            return (TimerState::Started, None);
        } else {
            return (TimerState::NotStarted, None);
        }
    }

    fn get_projects(&self) -> Vec<String> {
        // Search projects in odoo
        let projects = self
            .odoo_client
            .borrow_mut()
            .execute_kw(
                "project.project",
                "search_read",
                jvec![[["active", "=", true]]],
                jmap! { "fields": ["id", "display_name"] },
            )
            .send()
            .expect("Failed to execute Odoo kw request");

        // Collect their names
        let mut project_names = Vec::new();
        if let Some(data_array) = projects.data.as_array() {
            for item in data_array {
                if let Some(name) = item.get("display_name").and_then(|val| val.as_str()) {
                    project_names.push(name.to_string());
                }
            }
        }

        return project_names;
    }

    fn create_project(&self, project_name: &str) {
        println!("Not doing anything")
    }

    fn get_tasks_from_project(&self, project_name: &str) -> Vec<String> {
        // Search projects in odoo
        let tasks = self
            .odoo_client
            .borrow_mut()
            .execute_kw(
                "project.task",
                "search_read",
                jvec![[
                    ["active", "=", true],
                    ["project_id.display_name", "=", project_name]
                ]],
                jmap! { "fields": ["id", "display_name"] },
            )
            .send()
            .expect("Failed to execute Odoo kw request");

        // Collect their names
        let mut tasks_names = Vec::new();
        if let Some(data_array) = tasks.data.as_array() {
            for item in data_array {
                if let Some(name) = item.get("display_name").and_then(|val| val.as_str()) {
                    tasks_names.push(name.to_string());
                }
            }
        }

        return tasks_names;
    }

    fn create_task(&self, project_name: &str, task_name: &str) {
        println!("Not doing anything")
    }

    fn start_timer_on_task(&self, project_name: &str, task_name: &str) -> Result<(), Error> {
        let contenido = format!(
            "{}\n{}\n{}",
            project_name,
            task_name,
            &Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        );
        fs::write("cualquier_nombre.txt", contenido).unwrap();
        Ok(())
    }

    fn end_timer_on_task(
        &self,
        entry_to_close_id: &i32,
        description_input: &str,
    ) -> Result<(), Error> {
        let filename = "cualquier_nombre.txt";
        if Path::new(filename).exists() {
            if let Ok(file_content) = fs::read_to_string(filename) {
                let lineas: Vec<&str> = file_content.lines().collect();
                if lineas.len() >= 3 {
                    let estado = (
                        lineas[0].to_string(),
                        lineas[1].to_string(),
                        lineas[2].to_string(),
                    );

                    

                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn link_task_2_task(&self, project_name: &str, task_parent_name: &str, task_child_name: &str) {
        println!("Not doing anything")
    }
}
