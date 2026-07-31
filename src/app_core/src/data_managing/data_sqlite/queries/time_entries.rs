use crate::data_managing::data_sqlite::models::{NewTimeEntry, TimeEntry, UpdateTimeEntry};
use crate::data_managing::data_sqlite::schema::{
    project_task_dependencies, projects, tasks, time_entries, time_entries::dsl::*,
};
use diesel::prelude::*;

pub fn find_incomplete(conn: &mut SqliteConnection) -> Option<TimeEntry> {
    return time_entries
        .filter(end_time.is_null())
        .first::<TimeEntry>(&mut *conn)
        .optional()
        .expect("Error checking if there's a time entry without end time");
}

pub fn create_time_entry(
    conn: &mut SqliteConnection,
    project_name: &str,
    task_name: &str,
    start_time_input: &str,
) -> Result<(), diesel::result::Error> {
    let target_task_id = project_task_dependencies::table
        .inner_join(projects::table)
        .inner_join(tasks::table)
        .filter(projects::name.eq(project_name))
        .filter(tasks::title.eq(task_name))
        .select(tasks::id)
        .first::<i32>(conn)
        .optional()?;

    let target_task_id_result = match target_task_id {
        Some(target) => target,
        None => return Err(diesel::result::Error::NotFound),
    };

    match diesel::insert_into(time_entries::table)
        .values(&NewTimeEntry {
            task_id: target_task_id_result,
            start_time: &start_time_input,
            end_time: None,
            description: None,
        })
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error al insertar: {}", e);
            Err(e)
        }
    }
}

pub fn close_time_entry(
    conn: &mut SqliteConnection,
    entry_to_close_id: &i32,
    end_time_input: &str,
    description_input: &str,
) -> Result<(), diesel::result::Error> {
    match diesel::update(time_entries.find(entry_to_close_id))
        .set(&UpdateTimeEntry {
            end_time: end_time_input,
            description: description_input,
        })
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error al insertar: {}", e);
            Err(e)
        }
    }
}
