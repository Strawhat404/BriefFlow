-- ============================================================
-- Migration 003: Store Hours, Reservations, Sales Tax
-- ============================================================

CREATE TABLE IF NOT EXISTS `store_hours` (
    `id`          BIGINT     NOT NULL AUTO_INCREMENT,
    `day_of_week` TINYINT    NOT NULL COMMENT '0=Sunday, 1=Monday, ..., 6=Saturday',
    `open_time`   TIME       NOT NULL,
    `close_time`  TIME       NOT NULL,
    `is_closed`   BOOLEAN    NOT NULL DEFAULT FALSE,
    PRIMARY KEY (`id`),
    INDEX `idx_store_hours_day` (`day_of_week`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `reservations` (
    `id`                BIGINT      NOT NULL AUTO_INCREMENT,
    `user_id`           BIGINT      NOT NULL,
    `pickup_slot_start` DATETIME    NOT NULL,
    `pickup_slot_end`   DATETIME    NOT NULL,
    `voucher_code`      VARCHAR(50) NOT NULL,
    `hold_expires_at`   DATETIME    NOT NULL,
    `status`            ENUM('Held','Confirmed','Expired','Canceled') NOT NULL DEFAULT 'Held',
    `created_at`        DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`        DATETIME    NULL     ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_reservations_voucher` (`voucher_code`),
    INDEX `idx_reservations_user` (`user_id`),
    INDEX `idx_reservations_status` (`status`),
    INDEX `idx_reservations_pickup` (`pickup_slot_start`, `pickup_slot_end`),
    INDEX `idx_reservations_hold_expires` (`hold_expires_at`),
    CONSTRAINT `fk_reservations_user`
        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `sales_tax_config` (
    `id`        BIGINT        NOT NULL AUTO_INCREMENT,
    `tax_name`  VARCHAR(100)  NOT NULL,
    `rate`      DECIMAL(5,4)  NOT NULL,
    `is_active` BOOLEAN       NOT NULL DEFAULT TRUE,
    PRIMARY KEY (`id`),
    INDEX `idx_sales_tax_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
