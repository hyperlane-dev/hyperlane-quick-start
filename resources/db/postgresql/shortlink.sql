CREATE TABLE shortlink (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    user_cookie VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_shortlink_user_cookie ON shortlink(user_cookie);
CREATE INDEX idx_shortlink_created_at ON shortlink(created_at);