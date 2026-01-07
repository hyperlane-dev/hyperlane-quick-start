CREATE TABLE tracking_record (
    id BIGSERIAL PRIMARY KEY,
    socket_addr VARCHAR(255) NOT NULL,
    headers TEXT NOT NULL,
    body TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);