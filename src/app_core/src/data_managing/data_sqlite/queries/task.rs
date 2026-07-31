use crate::data_managing::data_sqlite::models::NewTask;
use crate::data_managing::data_sqlite::schema::{
    project_task_dependencies, projects, tasks, tasks::dsl::tasks as other_task,
};
use diesel::prelude::*;

pub fn create_task(
    conn: &mut SqliteConnection,
    task_title: &str,
    task_description: Option<&str>,
    task_status_id: Option<i32>,
    task_position: Option<i32>,
    task_finish_at_initial: Option<&str>,
    task_finish_at: Option<&str>,
    task_created_at: &str,
    task_updated_at: &str,
    task_time_spent: Option<i32>,
) -> usize {
    diesel::insert_into(other_task)
        .values(&NewTask {
            title: task_title,
            description: task_description,
            status_id: task_status_id,
            position: task_position,
            finish_at_initial: task_finish_at_initial,
            finish_at: task_finish_at,
            created_at: task_created_at,
            updated_at: task_updated_at,
            time_spent: task_time_spent,
        })
        .execute(conn)
        .expect("Error storing new task")
}

pub fn get_tasks_from_project(
    conn: &mut SqliteConnection,
    parent_project_name: &str,
) -> Vec<String> {
    match tasks::dsl::tasks
        .inner_join(
            project_task_dependencies::dsl::project_task_dependencies
                .inner_join(projects::dsl::projects),
        )
        .filter(projects::dsl::name.eq(parent_project_name))
        .select(tasks::dsl::title)
        .load::<String>(conn)
    {
        Ok(task_list) => task_list,
        Err(err_msg) => panic!(
            "Error searching all tasks from project: {} \n {}",
            parent_project_name, err_msg
        ),
    }
}
