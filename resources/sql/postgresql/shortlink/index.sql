CREATE UNIQUE INDEX IF NOT EXISTS idx_shortlink_url ON shortlink (url);

CREATE INDEX IF NOT EXISTS idx_shortlink_created_at ON shortlink (created_at);