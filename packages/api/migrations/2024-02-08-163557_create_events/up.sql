CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    household_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    event_date TIMESTAMP NOT NULL,
    created_by UUID NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(household_id) REFERENCES households(id),
    FOREIGN KEY(created_by) REFERENCES users(id)
)