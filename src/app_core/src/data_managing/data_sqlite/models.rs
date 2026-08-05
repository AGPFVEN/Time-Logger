use super::schema::{project_task_dependencies, projects, tags, tasks, time_entries};
use diesel::prelude::*;

/* FOR THE FUTURE
    // --- Task Statuses ---

    #[derive(Queryable, Selectable, Debug)]
    #[diesel(table_name = task_statuses)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct TaskStatus {
        pub id: i32,
        pub key: String,
        pub label: String,
        pub is_closed: bool,
    }

    #[derive(Insertable)]
    #[diesel(table_name = task_statuses)]
    pub struct NewTaskStatus<'a> {
        pub key: &'a str,
        pub label: &'a str,
        pub is_closed: bool,
    }

    // --- Project Statuses ---

    #[derive(Queryable, Selectable, Debug)]
    #[diesel(table_name = project_statuses)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct ProjectStatus {
        pub id: i32,
        pub key: String,
        pub label: String,
        pub is_closed: bool,
    }

    #[derive(Insertable)]
    #[diesel(table_name = project_statuses)]
    pub struct NewProjectStatus<'a> {
        pub key: &'a str,
        pub label: &'a str,
        pub is_closed: bool,
    }

// --- Tags ---
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color_hex: Option<String>,
    pub created_at: String,
}
*/

// --- Projects ---

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status_id: Option<i32>,
    pub finish_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub time_spent: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub status_id: Option<i32>,
    pub finish_at: Option<&'a str>,
    pub created_at: &'a str,
    pub updated_at: &'a str,
    pub time_spent: Option<i32>,
}

// --- Tasks ---

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status_id: Option<i32>,
    pub position: Option<i32>,
    pub finish_at_initial: Option<String>,
    pub finish_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub time_spent: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status_id: Option<i32>,
    pub position: Option<i32>,
    pub finish_at_initial: Option<&'a str>,
    pub finish_at: Option<&'a str>,
    pub created_at: &'a str,
    pub updated_at: &'a str,
    pub time_spent: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub name: &'a str,
    pub color_hex: Option<&'a str>,
    pub created_at: &'a str,
}

// --- Time Entries ---
#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(table_name = time_entries)]
pub struct TimeEntry {
    pub id: i32,
    pub task_id: i32,
    pub start_time: String,
    pub end_time: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub duration: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = time_entries)]
pub struct NewTimeEntry<'a> {
    pub task_id: i32,
    pub start_time: &'a str,
    pub end_time: Option<&'a str>,
    pub description: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = time_entries)]
pub struct UpdateTimeEntry<'a> {
    pub end_time: &'a str,
    pub duration: &'a i32,
    pub description: &'a str,
}

// --- Link Tasks to Projects ---
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = project_task_dependencies)]
#[diesel(primary_key(project_id, task_id))]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(Task))]
pub struct ProjectTaskDependency {
    pub project_id: i32,
    pub task_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::project_task_dependencies)]
pub struct NewProjectTaskDependency {
    pub project_id: i32,
    pub task_id: i32,
}

// --- Link Tasks to Tasks ---
#[derive(Insertable)]
#[diesel(table_name = super::schema::task_task_dependencies)]
pub struct NewTaskTaskDependency {
    pub parent_id: i32,
    pub child_id: i32,
}
