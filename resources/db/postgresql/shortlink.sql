CREATE TABLE shortlink (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_shortlink_url ON shortlink (url);

CREATE INDEX IF NOT EXISTS idx_shortlink_created_at ON shortlink (created_at);