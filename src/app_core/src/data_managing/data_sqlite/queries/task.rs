use crate::data_managing::data_sqlite::models::{NewTask, NewTaskTaskDependency, Task};
use crate::data_managing::data_sqlite::schema::{
    project_task_dependencies, projects, task_task_dependencies, tasks,
    tasks::dsl::tasks as other_task,
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

pub fn link_task_2_task(
    conn: &mut SqliteConnection,
    project_name: &str,
    task_parent_name: &str,
    task_child_name: &str,
) {
    let task_parent = search_task(conn, project_name, task_parent_name);
    let task_child = search_task(conn, project_name, task_child_name);

    let new_dependency = NewTaskTaskDependency {
        parent_id: task_parent.id,
        child_id: task_child.id,
    };

    diesel::insert_into(task_task_dependencies::table)
        .values(&new_dependency)
        .execute(conn)
        .expect("Error linking task to project");
}

fn search_task(conn: &mut SqliteConnection, project_name: &str, task_name: &str) -> Task {
    match tasks::dsl::tasks
        .inner_join(
            project_task_dependencies::dsl::project_task_dependencies
                .inner_join(projects::dsl::projects),
        )
        .filter(projects::dsl::name.eq(project_name))
        .filter(tasks::dsl::title.eq(task_name))
        .select(tasks::all_columns)
        .first(conn)
        .optional()
    {
        Ok(task_res) => match task_res {
            Some(task) => return task,
            None => panic!(
                "There's no task named {} from project {}",
                task_name, project_name
            ),
        },
        Err(err_msg) => panic!(
            "An error occurred while searching in SQLite db when:\nsearching task: {}\nfrom project: {}\n{}",
            task_name, project_name, err_msg
        ),
    }
}
