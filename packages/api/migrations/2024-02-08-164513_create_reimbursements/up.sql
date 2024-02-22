CREATE TYPE reimbursement_status AS ENUM ('Pending', 'Completed');
CREATE TABLE IF NOT EXISTS reimbursements(
    id SERIAL PRIMARY KEY,
    expense_id INTEGER NOT NULL,
    payer_id UUID NOT NULL,
    beneficiary_id UUID NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    status reimbursement_status NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (expense_id) REFERENCES expenses(id),
    FOREIGN KEY (payer_id) REFERENCES users(id),
    FOREIGN KEY (beneficiary_id) REFERENCES users(id)
)