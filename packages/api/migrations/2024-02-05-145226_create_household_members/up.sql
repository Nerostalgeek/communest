CREATE TABLE household_members (
    member_id SERIAL PRIMARY KEY,
    household_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    role VARCHAR(255),
    date_added TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
);