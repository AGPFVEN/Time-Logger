use crate::data_managing::data_sqlite::models::{
    NewProject, NewProjectTaskDependency, Project, Task,
};
use crate::data_managing::data_sqlite::schema::{
    project_task_dependencies, projects, projects::dsl::*, tasks,
};

use diesel::prelude::*;

pub fn get_all_projects(conn: &mut SqliteConnection) -> Vec<String> {
    projects
        .select(name)
        .load::<String>(conn)
        .expect("Error loading projects")
}

pub fn create_project(
    conn: &mut SqliteConnection,
    project_name: &str,
    project_description: Option<&str>,
    project_status_id: Option<i32>,
    project_finish_at: Option<&str>,
    project_created_at: &str,
    project_updated_at: &str,
    project_time_spent: Option<i32>,
) -> usize {
    diesel::insert_into(projects)
        .values(&NewProject {
            name: project_name,
            description: project_description,
            status_id: project_status_id,
            finish_at: project_finish_at,
            created_at: project_created_at,
            updated_at: project_updated_at,
            time_spent: project_time_spent,
        })
        .execute(conn)
        .expect("Error storing new project")
}

pub fn link_task_to_project(conn: &mut SqliteConnection, project_name: &str, task_name: &str) {
    let project = projects::table
        .filter(projects::name.eq(project_name))
        .first::<Project>(conn);

    let task = tasks::table
        .filter(tasks::title.eq(task_name))
        .first::<Task>(conn);

    let new_dependency = NewProjectTaskDependency {
        project_id: project.unwrap().id,
        task_id: task.unwrap().id,
    };

    diesel::insert_into(project_task_dependencies::table)
        .values(&new_dependency)
        .execute(conn)
        .expect("Error linking task to project");
}
