CREATE UNIQUE INDEX IF NOT EXISTS idx_github_pages_owner_repository ON github_pages (owner, repository);

CREATE INDEX IF NOT EXISTS idx_github_pages_last_synced_at ON github_pages (last_synced_at);