-- ============================================================
-- Migration 010: Fix voucher hash column lengths
--
-- SHA-256 hex digests are 64 characters.  The original schema
-- defined both `reservations.voucher_code` and `vouchers.code`
-- as VARCHAR(50), which silently truncates the hash and breaks
-- lookup equality.  Widen both columns to VARCHAR(64).
-- ============================================================

ALTER TABLE `reservations`
    MODIFY COLUMN `voucher_code` VARCHAR(64) NOT NULL;

ALTER TABLE `vouchers`
    MODIFY COLUMN `code` VARCHAR(64) NOT NULL;
