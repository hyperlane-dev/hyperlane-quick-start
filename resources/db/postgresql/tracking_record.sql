CREATE TABLE tracking_record (
    id BIGSERIAL PRIMARY KEY,
    socket_addr VARCHAR(255) NOT NULL,
    headers TEXT NOT NULL,
    body TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tracking_socket_addr ON tracking_record (socket_addr);

CREATE INDEX IF NOT EXISTS idx_tracking_timestamp ON tracking_record (timestamp);

CREATE INDEX IF NOT EXISTS idx_tracking_created_at ON tracking_record (created_at);