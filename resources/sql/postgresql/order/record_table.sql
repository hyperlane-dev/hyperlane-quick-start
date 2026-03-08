CREATE TABLE IF NOT EXISTS order_record (
    id SERIAL PRIMARY KEY,
    bill_no VARCHAR(32) UNIQUE NOT NULL,
    user_id INTEGER NOT NULL REFERENCES order_user (id),
    amount DECIMAL(15, 2) NOT NULL,
    category VARCHAR(50) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL,
    description TEXT,
    bill_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);