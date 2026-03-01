CREATE UNIQUE INDEX IF NOT EXISTS idx_order_user_username ON order_user (username);

CREATE INDEX IF NOT EXISTS idx_order_user_status ON order_user (status);

CREATE INDEX IF NOT EXISTS idx_order_record_user_id ON order_record (user_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_order_record_bill_no ON order_record (bill_no);

CREATE INDEX IF NOT EXISTS idx_order_record_bill_date ON order_record (bill_date);