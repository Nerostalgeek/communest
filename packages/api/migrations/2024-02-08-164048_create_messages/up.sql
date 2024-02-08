CREATE TABLE IF NOT EXISTS messages (
    message_id SERIAL PRIMARY KEY,
    sender_id INTEGER,
    household_id INTEGER,
    content TEXT,
    sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sender_id) REFERENCES users(user_id),
    FOREIGN KEY (household_id) REFERENCES household(household_id)
)