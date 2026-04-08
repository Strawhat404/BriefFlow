CREATE TABLE IF NOT EXISTS sessions (
    session_id VARCHAR(64) PRIMARY KEY,
    user_id BIGINT NOT NULL,
    user_agent VARCHAR(500),
    ip_address VARCHAR(45),
    last_activity DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rotated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_sessions_user (user_id),
    INDEX idx_sessions_activity (last_activity)
);

-- Add encrypted_voucher_code column to vouchers table
ALTER TABLE vouchers ADD COLUMN encrypted_code VARCHAR(500) NULL AFTER code;

-- Add password_changed_at to users
ALTER TABLE users ADD COLUMN password_changed_at DATETIME NULL AFTER password_hash;
