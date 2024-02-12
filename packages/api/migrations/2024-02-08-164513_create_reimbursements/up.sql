CREATE TYPE reimbursement_status AS ENUM ('Pending', 'Completed');
CREATE TABLE IF NOT EXISTS reimbursements(
    reimbursement_id SERIAL PRIMARY KEY,
    expense_id INTEGER NOT NULL,
    payer_id INTEGER NOT NULL,
    beneficiary_id INTEGER NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    status reimbursement_status NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (expense_id) REFERENCES expenses(expense_id),
    FOREIGN KEY (payer_id) REFERENCES users(user_id),
    FOREIGN KEY (beneficiary_id) REFERENCES users(user_id)
)