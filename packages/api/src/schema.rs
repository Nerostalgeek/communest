// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "reimbursement_status"))]
    pub struct ReimbursementStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "task_status"))]
    pub struct TaskStatus;
}

diesel::table! {
    events (id) {
        id -> Int4,
        household_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        event_date -> Timestamp,
        created_by -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    expenses (id) {
        id -> Int4,
        amount -> Numeric,
        #[max_length = 255]
        description -> Varchar,
        payer_id -> Int4,
        household_id -> Int4,
        expense_date -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    household_members (id) {
        id -> Int4,
        household_id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        nickname -> Nullable<Varchar>,
        #[max_length = 255]
        role -> Nullable<Varchar>,
        date_added -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    households (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        created_by -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        sender_id -> Int4,
        household_id -> Int4,
        content -> Text,
        sent_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ReimbursementStatus;

    reimbursements (id) {
        id -> Int4,
        expense_id -> Int4,
        payer_id -> Int4,
        beneficiary_id -> Int4,
        amount -> Numeric,
        status -> ReimbursementStatus,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TaskStatus;

    tasks (id) {
        id -> Int4,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        assigned_to -> Nullable<Int4>,
        household_id -> Nullable<Int4>,
        due_date -> Nullable<Timestamp>,
        status -> Nullable<TaskStatus>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(events -> households (household_id));
diesel::joinable!(events -> users (created_by));
diesel::joinable!(expenses -> households (household_id));
diesel::joinable!(expenses -> users (payer_id));
diesel::joinable!(household_members -> households (household_id));
diesel::joinable!(household_members -> users (user_id));
diesel::joinable!(households -> users (created_by));
diesel::joinable!(messages -> households (household_id));
diesel::joinable!(messages -> users (sender_id));
diesel::joinable!(reimbursements -> expenses (expense_id));
diesel::joinable!(tasks -> households (household_id));
diesel::joinable!(tasks -> users (assigned_to));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    expenses,
    household_members,
    households,
    messages,
    reimbursements,
    tasks,
    users,
);
