-- ============================================================
-- Migration 002: Products (SPU, Option Groups, SKU)
-- ============================================================

CREATE TABLE IF NOT EXISTS `spu` (
    `id`                BIGINT        NOT NULL AUTO_INCREMENT,
    `name_en`           VARCHAR(200)  NOT NULL,
    `name_zh`           VARCHAR(200)  NOT NULL,
    `description_en`    TEXT          NULL,
    `description_zh`    TEXT          NULL,
    `category`          VARCHAR(100)  NULL,
    `image_url`         VARCHAR(500)  NULL,
    `base_price`        DECIMAL(10,2) NOT NULL,
    `prep_time_minutes` INT           NOT NULL DEFAULT 5,
    `is_active`         BOOLEAN       NOT NULL DEFAULT TRUE,
    `created_at`        DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`        DATETIME      NULL     ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_spu_category` (`category`),
    INDEX `idx_spu_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `option_groups` (
    `id`          BIGINT       NOT NULL AUTO_INCREMENT,
    `spu_id`      BIGINT       NOT NULL,
    `name_en`     VARCHAR(200) NOT NULL,
    `name_zh`     VARCHAR(200) NOT NULL,
    `is_required` BOOLEAN      NOT NULL DEFAULT TRUE,
    `sort_order`  INT          NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    INDEX `idx_option_groups_spu` (`spu_id`),
    CONSTRAINT `fk_option_groups_spu`
        FOREIGN KEY (`spu_id`) REFERENCES `spu` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `option_values` (
    `id`          BIGINT        NOT NULL AUTO_INCREMENT,
    `group_id`    BIGINT        NOT NULL,
    `label_en`    VARCHAR(200)  NOT NULL,
    `label_zh`    VARCHAR(200)  NOT NULL,
    `price_delta` DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    `is_default`  BOOLEAN       NOT NULL DEFAULT FALSE,
    `sort_order`  INT           NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    INDEX `idx_option_values_group` (`group_id`),
    CONSTRAINT `fk_option_values_group`
        FOREIGN KEY (`group_id`) REFERENCES `option_groups` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `sku` (
    `id`              BIGINT        NOT NULL AUTO_INCREMENT,
    `spu_id`          BIGINT        NOT NULL,
    `sku_code`        VARCHAR(100)  NOT NULL,
    `price`           DECIMAL(10,2) NOT NULL,
    `stock_quantity`  INT           NOT NULL DEFAULT 999,
    `is_active`       BOOLEAN       NOT NULL DEFAULT TRUE,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_sku_code` (`sku_code`),
    INDEX `idx_sku_spu` (`spu_id`),
    CONSTRAINT `fk_sku_spu`
        FOREIGN KEY (`spu_id`) REFERENCES `spu` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `sku_option_values` (
    `sku_id`          BIGINT NOT NULL,
    `option_value_id` BIGINT NOT NULL,
    PRIMARY KEY (`sku_id`, `option_value_id`),
    INDEX `idx_sku_option_values_option` (`option_value_id`),
    CONSTRAINT `fk_sku_option_values_sku`
        FOREIGN KEY (`sku_id`) REFERENCES `sku` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_sku_option_values_option`
        FOREIGN KEY (`option_value_id`) REFERENCES `option_values` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
