CREATE TABLE IF NOT EXISTS order_record_image (
    id SERIAL PRIMARY KEY,
    record_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    original_name VARCHAR(255),
    mime_type VARCHAR(100) NOT NULL,
    file_size INTEGER NOT NULL,
    file_data BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);