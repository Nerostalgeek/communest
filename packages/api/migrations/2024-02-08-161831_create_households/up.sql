CREATE TABLE IF NOT EXISTS households (
    household_id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    address VARCHAR(255),
    created_by INTEGER,
    FOREIGN KEY (created_by) REFERENCES users(user_id)
)