CREATE UNIQUE INDEX IF NOT EXISTS idx_account_booking_user_username ON account_booking_user (username);

CREATE INDEX IF NOT EXISTS idx_account_booking_user_status ON account_booking_user (status);

CREATE INDEX IF NOT EXISTS idx_account_booking_record_user_id ON account_booking_record (user_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_account_booking_record_bill_no ON account_booking_record (bill_no);

CREATE INDEX IF NOT EXISTS idx_account_booking_record_bill_date ON account_booking_record (bill_date);