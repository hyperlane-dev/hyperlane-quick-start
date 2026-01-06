CREATE INDEX IF NOT EXISTS idx_chat_history_session_id ON chat_history(session_id);
CREATE INDEX IF NOT EXISTS idx_chat_history_created_at ON chat_history(created_at);
CREATE UNIQUE INDEX IF NOT EXISTS idx_shortlink_url ON shortlink(url);
CREATE INDEX IF NOT EXISTS idx_shortlink_created_at ON shortlink(created_at);
