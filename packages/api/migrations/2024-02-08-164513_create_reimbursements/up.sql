CREATE TYPE status_enum AS ENUM ('Pending', 'Completed');
CREATE TABLE IF NOT EXISTS reimbursements(
    reimbursement_id SERIAL PRIMARY KEY,
    expense_id INTEGER,
    payer_id INTEGER,
    beneficiary_id INTEGER,
    amount DECIMAL,
    status status_enum,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (expense_id) REFERENCES expenses(expense_id),
    FOREIGN KEY (payer_id) REFERENCES users(user_id),
    FOREIGN KEY (beneficiary_id) REFERENCES users(user_id)
)