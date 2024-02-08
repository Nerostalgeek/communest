CREATE TYPE status_enum AS ENUM (
    'Pending',
    'InProgress',
    'Completed',
    'Cancelled',
    'Approved'
);
CREATE TABLE IF NOT EXISTS tasks (
    task_id: SERIAL PRIMARY KEY,
    description: VARCHAR(255),
    assigned_to: INTEGER,
    household_id: INTEGER,
    due_date: TIMESTAMP,
    status: status_enum,
    FOREIGN KEY (assigned_to) REFERENCES users(user_id),
    FOREIGN KEY (household_id) REFERENCES households(household_id)
)