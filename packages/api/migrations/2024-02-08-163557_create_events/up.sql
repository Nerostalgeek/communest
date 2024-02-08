CREATE TABLE IF NOT EXISTS events (
    event_id: SERIAL PRIMARY KEY,
    household_id: INTEGER,
    title: VARCHAR(255),
    description: VARCHAR(255),
    event_date: TIMESTAMP,
    created_by: INTEGER,
    FOREIGN KEY(household_id) REFERENCES household(households_id),
    FOREIGN KEY(created_by) REFERENCES users(user_id)
)