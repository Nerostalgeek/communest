CREATE TABLE IF NOT EXISTS household_members (
    member_id SERIAL PRIMARY KEY,
    household_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    nickname VARCHAR(255),
    role VARCHAR(255),
    date_added TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (household_id) REFERENCES households(household_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);