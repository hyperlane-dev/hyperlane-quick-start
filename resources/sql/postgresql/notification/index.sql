CREATE INDEX IF NOT EXISTS idx_notification_user_id ON notification (user_id);

CREATE INDEX IF NOT EXISTS idx_notification_is_read ON notification (is_read);

CREATE INDEX IF NOT EXISTS idx_notification_type ON notification (notification_type);

CREATE INDEX IF NOT EXISTS idx_notification_created_at ON notification (created_at DESC);

CREATE INDEX IF NOT EXISTS idx_notification_user_read ON notification (user_id, is_read);