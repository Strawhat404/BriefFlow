-- ============================================================
-- Migration 004: Cart, Orders, Fulfillment, Vouchers
-- ============================================================

CREATE TABLE IF NOT EXISTS `carts` (
    `id`         BIGINT   NOT NULL AUTO_INCREMENT,
    `user_id`    BIGINT   NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME NULL     ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_carts_user` (`user_id`),
    CONSTRAINT `fk_carts_user`
        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `cart_items` (
    `id`         BIGINT        NOT NULL AUTO_INCREMENT,
    `cart_id`    BIGINT        NOT NULL,
    `sku_id`     BIGINT        NOT NULL,
    `quantity`   INT           NOT NULL DEFAULT 1,
    `unit_price` DECIMAL(10,2) NOT NULL,
    `created_at` DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_cart_items_cart` (`cart_id`),
    INDEX `idx_cart_items_sku` (`sku_id`),
    CONSTRAINT `fk_cart_items_cart`
        FOREIGN KEY (`cart_id`) REFERENCES `carts` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_cart_items_sku`
        FOREIGN KEY (`sku_id`) REFERENCES `sku` (`id`)
        ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `cart_item_options` (
    `id`              BIGINT NOT NULL AUTO_INCREMENT,
    `cart_item_id`    BIGINT NOT NULL,
    `option_value_id` BIGINT NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `idx_cart_item_options_item` (`cart_item_id`),
    INDEX `idx_cart_item_options_option` (`option_value_id`),
    CONSTRAINT `fk_cart_item_options_item`
        FOREIGN KEY (`cart_item_id`) REFERENCES `cart_items` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_cart_item_options_option`
        FOREIGN KEY (`option_value_id`) REFERENCES `option_values` (`id`)
        ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `orders` (
    `id`             BIGINT        NOT NULL AUTO_INCREMENT,
    `user_id`        BIGINT        NOT NULL,
    `reservation_id` BIGINT        NULL,
    `order_number`   VARCHAR(50)   NOT NULL,
    `subtotal`       DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    `tax_amount`     DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    `total`          DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    `status`         ENUM('Pending','Accepted','InPrep','Ready','PickedUp','Canceled') NOT NULL DEFAULT 'Pending',
    `created_at`     DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`     DATETIME      NULL     ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_orders_number` (`order_number`),
    INDEX `idx_orders_user` (`user_id`),
    INDEX `idx_orders_reservation` (`reservation_id`),
    INDEX `idx_orders_status` (`status`),
    INDEX `idx_orders_created` (`created_at`),
    CONSTRAINT `fk_orders_user`
        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT `fk_orders_reservation`
        FOREIGN KEY (`reservation_id`) REFERENCES `reservations` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `order_items` (
    `id`         BIGINT        NOT NULL AUTO_INCREMENT,
    `order_id`   BIGINT        NOT NULL,
    `sku_id`     BIGINT        NOT NULL,
    `quantity`   INT           NOT NULL DEFAULT 1,
    `unit_price` DECIMAL(10,2) NOT NULL,
    `item_total` DECIMAL(10,2) NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `idx_order_items_order` (`order_id`),
    INDEX `idx_order_items_sku` (`sku_id`),
    CONSTRAINT `fk_order_items_order`
        FOREIGN KEY (`order_id`) REFERENCES `orders` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_order_items_sku`
        FOREIGN KEY (`sku_id`) REFERENCES `sku` (`id`)
        ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `order_item_options` (
    `id`              BIGINT        NOT NULL AUTO_INCREMENT,
    `order_item_id`   BIGINT        NOT NULL,
    `option_value_id` BIGINT        NOT NULL,
    `option_label`    VARCHAR(200)  NOT NULL,
    `price_delta`     DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    PRIMARY KEY (`id`),
    INDEX `idx_order_item_options_item` (`order_item_id`),
    INDEX `idx_order_item_options_option` (`option_value_id`),
    CONSTRAINT `fk_order_item_options_item`
        FOREIGN KEY (`order_item_id`) REFERENCES `order_items` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_order_item_options_option`
        FOREIGN KEY (`option_value_id`) REFERENCES `option_values` (`id`)
        ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `fulfillment_events` (
    `id`                 BIGINT      NOT NULL AUTO_INCREMENT,
    `order_id`           BIGINT      NOT NULL,
    `from_status`        VARCHAR(50) NULL,
    `to_status`          VARCHAR(50) NOT NULL,
    `changed_by_user_id` BIGINT      NOT NULL,
    `notes`              TEXT        NULL,
    `created_at`         DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_fulfillment_order` (`order_id`),
    INDEX `idx_fulfillment_changed_by` (`changed_by_user_id`),
    CONSTRAINT `fk_fulfillment_order`
        FOREIGN KEY (`order_id`) REFERENCES `orders` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_fulfillment_changed_by`
        FOREIGN KEY (`changed_by_user_id`) REFERENCES `users` (`id`)
        ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `vouchers` (
    `id`               BIGINT      NOT NULL AUTO_INCREMENT,
    `reservation_id`   BIGINT      NOT NULL,
    `order_id`         BIGINT      NULL,
    `code`             VARCHAR(50) NOT NULL,
    `scanned_at`       DATETIME    NULL,
    `scanned_by_user_id` BIGINT    NULL,
    `mismatch_flag`    BOOLEAN     NOT NULL DEFAULT FALSE,
    `mismatch_reason`  TEXT        NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_vouchers_code` (`code`),
    INDEX `idx_vouchers_reservation` (`reservation_id`),
    INDEX `idx_vouchers_order` (`order_id`),
    INDEX `idx_vouchers_scanned_by` (`scanned_by_user_id`),
    CONSTRAINT `fk_vouchers_reservation`
        FOREIGN KEY (`reservation_id`) REFERENCES `reservations` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_vouchers_order`
        FOREIGN KEY (`order_id`) REFERENCES `orders` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT `fk_vouchers_scanned_by`
        FOREIGN KEY (`scanned_by_user_id`) REFERENCES `users` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
