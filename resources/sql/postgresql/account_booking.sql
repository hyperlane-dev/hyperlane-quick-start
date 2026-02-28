CREATE TABLE IF NOT EXISTS account_booking_user (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(100),
    email VARCHAR(100),
    phone VARCHAR(20),
    role VARCHAR(20) NOT NULL DEFAULT 'user',
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS account_booking_record (
    id SERIAL PRIMARY KEY,
    bill_no VARCHAR(32) UNIQUE NOT NULL,
    user_id INTEGER NOT NULL REFERENCES account_booking_user (id),
    amount DECIMAL(15, 2) NOT NULL,
    category VARCHAR(50) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL,
    description TEXT,
    bill_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);