CREATE TABLE IF NOT EXISTS expenses (
    expense_id SERIAL PRIMARY KEY,
    amount DECIMAL,
    description VARCHAR(255),
    payer_id INTEGER,
    household_id INTEGER,
    expense_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (payer_id) REFERENCES users(user_id),
    FOREIGN KEY (household_id) REFERENCES households(household_id)
)