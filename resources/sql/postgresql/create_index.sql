CREATE INDEX IF NOT EXISTS idx_chat_history_session_id ON chat_history (session_id);

CREATE INDEX IF NOT EXISTS idx_chat_history_created_at ON chat_history (created_at);

CREATE INDEX IF NOT EXISTS idx_tracking_socket_addr ON tracking_record (socket_addr);

CREATE INDEX IF NOT EXISTS idx_tracking_timestamp ON tracking_record (timestamp);

CREATE INDEX IF NOT EXISTS idx_tracking_created_at ON tracking_record (created_at);

CREATE UNIQUE INDEX IF NOT EXISTS idx_shortlink_url ON shortlink (url);

CREATE INDEX IF NOT EXISTS idx_shortlink_created_at ON shortlink (created_at);