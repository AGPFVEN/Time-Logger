// @generated automatically by Diesel CLI.

diesel::table! {
    project_statuses (id) {
        id -> Integer,
        key -> Text,
        label -> Text,
        is_closed -> Bool,
    }
}

diesel::table! {
    project_tags (project_id, tag_id) {
        project_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    project_task_dependencies (project_id, task_id) {
        project_id -> Integer,
        task_id -> Integer,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        status_id -> Nullable<Integer>,
        finish_at -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        time_spent -> Nullable<Integer>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        color_hex -> Nullable<Text>,
        created_at -> Text,
    }
}

diesel::table! {
    task_statuses (id) {
        id -> Integer,
        key -> Text,
        label -> Text,
        is_closed -> Bool,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    task_task_dependencies (parent_id, child_id) {
        parent_id -> Integer,
        child_id -> Integer,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status_id -> Nullable<Integer>,
        position -> Nullable<Integer>,
        finish_at_initial -> Nullable<Text>,
        finish_at -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        time_spent -> Nullable<Integer>,
    }
}

diesel::table! {
    time_entries (id) {
        id -> Integer,
        task_id -> Integer,
        start_time -> Text,
        end_time -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Text,
        duration -> Nullable<Integer>,
    }
}

diesel::joinable!(project_tags -> projects (project_id));
diesel::joinable!(project_tags -> tags (tag_id));
diesel::joinable!(project_task_dependencies -> projects (project_id));
diesel::joinable!(project_task_dependencies -> tasks (task_id));
diesel::joinable!(projects -> project_statuses (status_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(tasks -> task_statuses (status_id));
diesel::joinable!(time_entries -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    project_statuses,
    project_tags,
    project_task_dependencies,
    projects,
    tags,
    task_statuses,
    task_tags,
    task_task_dependencies,
    tasks,
    time_entries,
);
