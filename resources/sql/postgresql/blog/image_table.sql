CREATE TABLE IF NOT EXISTS blog_image (
    id SERIAL PRIMARY KEY,
    post_id INTEGER DEFAULT 0,
    user_id INTEGER NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    original_name VARCHAR(255),
    mime_type VARCHAR(100) NOT NULL,
    file_size INTEGER NOT NULL DEFAULT 0,
    file_data BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);