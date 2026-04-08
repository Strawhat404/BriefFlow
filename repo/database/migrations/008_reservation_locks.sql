CREATE TABLE IF NOT EXISTS reservation_locks (
    lock_key VARCHAR(255) PRIMARY KEY,
    user_id BIGINT NOT NULL,
    reservation_id BIGINT NULL,
    sku_id BIGINT NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    acquired_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    released BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (sku_id) REFERENCES sku(id),
    INDEX idx_lock_expires (expires_at, released),
    INDEX idx_lock_sku (sku_id)
);

-- Add stock tracking index to SKU
ALTER TABLE sku ADD INDEX idx_sku_stock (stock_quantity, is_active);
