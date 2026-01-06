CREATE INDEX IF NOT EXISTS idx_chat_history_session_id ON chat_history(session_id);
CREATE INDEX IF NOT EXISTS idx_chat_history_created_at ON chat_history(created_at);
CREATE INDEX IF NOT EXISTS idx_shortlink_user_cookie ON shortlink(user_cookie);
CREATE INDEX IF NOT EXISTS idx_shortlink_created_at ON shortlink(created_at);
