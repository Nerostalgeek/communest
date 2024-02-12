CREATE TABLE IF NOT EXISTS events (
    event_id SERIAL PRIMARY KEY,
    household_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    event_date TIMESTAMP NOT NULL,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(household_id) REFERENCES households(household_id),
    FOREIGN KEY(created_by) REFERENCES users(user_id)
)