CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_user_username ON auth_user (username);

CREATE INDEX IF NOT EXISTS idx_auth_user_status ON auth_user (status);