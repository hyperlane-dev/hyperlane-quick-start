CREATE TABLE IF NOT EXISTS github_pages (
    id SERIAL PRIMARY KEY,
    owner TEXT NOT NULL,
    repository TEXT NOT NULL,
    base_url TEXT NOT NULL,
    last_synced_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);