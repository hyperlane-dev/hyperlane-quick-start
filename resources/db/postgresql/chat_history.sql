CREATE TABLE chat_history (
    id BIGSERIAL PRIMARY KEY,
    session_id VARCHAR(255) NOT NULL,
    sender_name VARCHAR(255) NOT NULL,
    sender_type VARCHAR(50) NOT NULL,
    message_type VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_chat_history_session_id ON chat_history (session_id);

CREATE INDEX IF NOT EXISTS idx_chat_history_created_at ON chat_history (created_at);