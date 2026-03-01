CREATE INDEX IF NOT EXISTS idx_tracking_socket_addr ON tracking_record (socket_addr);

CREATE INDEX IF NOT EXISTS idx_tracking_timestamp ON tracking_record (timestamp);

CREATE INDEX IF NOT EXISTS idx_tracking_created_at ON tracking_record (created_at);