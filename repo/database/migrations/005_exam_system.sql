-- ============================================================
-- Migration 005: Exam / Quiz System
-- ============================================================

CREATE TABLE IF NOT EXISTS `subjects` (
    `id`             BIGINT       NOT NULL AUTO_INCREMENT,
    `name_en`        VARCHAR(200) NOT NULL,
    `name_zh`        VARCHAR(200) NOT NULL,
    `description_en` TEXT         NULL,
    `description_zh` TEXT         NULL,
    `created_at`     DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_subjects_name_en` (`name_en`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `chapters` (
    `id`         BIGINT       NOT NULL AUTO_INCREMENT,
    `subject_id` BIGINT       NOT NULL,
    `name_en`    VARCHAR(200) NOT NULL,
    `name_zh`    VARCHAR(200) NOT NULL,
    `sort_order` INT          NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    INDEX `idx_chapters_subject` (`subject_id`),
    CONSTRAINT `fk_chapters_subject`
        FOREIGN KEY (`subject_id`) REFERENCES `subjects` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `questions` (
    `id`               BIGINT NOT NULL AUTO_INCREMENT,
    `subject_id`       BIGINT NOT NULL,
    `chapter_id`       BIGINT NULL,
    `difficulty`       ENUM('Easy','Medium','Hard') NOT NULL DEFAULT 'Medium',
    `question_text_en` TEXT   NOT NULL,
    `question_text_zh` TEXT   NULL,
    `explanation_en`   TEXT   NULL,
    `explanation_zh`   TEXT   NULL,
    `question_type`    ENUM('SingleChoice','MultipleChoice','TrueFalse') NOT NULL DEFAULT 'SingleChoice',
    `imported_from`    VARCHAR(500) NULL,
    `created_by`       BIGINT NULL,
    `created_at`       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`       DATETIME NULL     ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_questions_subject` (`subject_id`),
    INDEX `idx_questions_chapter` (`chapter_id`),
    INDEX `idx_questions_difficulty` (`difficulty`),
    INDEX `idx_questions_type` (`question_type`),
    INDEX `idx_questions_created_by` (`created_by`),
    CONSTRAINT `fk_questions_subject`
        FOREIGN KEY (`subject_id`) REFERENCES `subjects` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_questions_chapter`
        FOREIGN KEY (`chapter_id`) REFERENCES `chapters` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT `fk_questions_created_by`
        FOREIGN KEY (`created_by`) REFERENCES `users` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `question_options` (
    `id`          BIGINT      NOT NULL AUTO_INCREMENT,
    `question_id` BIGINT      NOT NULL,
    `label`       VARCHAR(10) NOT NULL,
    `content_en`  TEXT        NOT NULL,
    `content_zh`  TEXT        NULL,
    `is_correct`  BOOLEAN     NOT NULL DEFAULT FALSE,
    `sort_order`  INT         NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    INDEX `idx_question_options_question` (`question_id`),
    CONSTRAINT `fk_question_options_question`
        FOREIGN KEY (`question_id`) REFERENCES `questions` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `exam_versions` (
    `id`                 BIGINT       NOT NULL AUTO_INCREMENT,
    `title_en`           VARCHAR(200) NOT NULL,
    `title_zh`           VARCHAR(200) NOT NULL,
    `subject_id`         BIGINT       NULL,
    `chapter_id`         BIGINT       NULL,
    `difficulty`         ENUM('Easy','Medium','Hard','Mixed') NOT NULL DEFAULT 'Mixed',
    `question_count`     INT          NOT NULL,
    `time_limit_minutes` INT          NOT NULL,
    `created_by`         BIGINT       NULL,
    `created_at`         DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_exam_versions_subject` (`subject_id`),
    INDEX `idx_exam_versions_chapter` (`chapter_id`),
    INDEX `idx_exam_versions_created_by` (`created_by`),
    CONSTRAINT `fk_exam_versions_subject`
        FOREIGN KEY (`subject_id`) REFERENCES `subjects` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT `fk_exam_versions_chapter`
        FOREIGN KEY (`chapter_id`) REFERENCES `chapters` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT `fk_exam_versions_created_by`
        FOREIGN KEY (`created_by`) REFERENCES `users` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `exam_version_questions` (
    `exam_version_id` BIGINT NOT NULL,
    `question_id`     BIGINT NOT NULL,
    `sort_order`      INT    NOT NULL DEFAULT 0,
    PRIMARY KEY (`exam_version_id`, `question_id`),
    INDEX `idx_evq_question` (`question_id`),
    CONSTRAINT `fk_evq_exam_version`
        FOREIGN KEY (`exam_version_id`) REFERENCES `exam_versions` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_evq_question`
        FOREIGN KEY (`question_id`) REFERENCES `questions` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `exam_attempts` (
    `id`              BIGINT   NOT NULL AUTO_INCREMENT,
    `user_id`         BIGINT   NOT NULL,
    `exam_version_id` BIGINT   NOT NULL,
    `started_at`      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `finished_at`     DATETIME NULL,
    `score`           DECIMAL(5,2) NULL,
    `total_questions`  INT     NOT NULL,
    `correct_count`   INT      NOT NULL DEFAULT 0,
    `status`          ENUM('InProgress','Completed','Abandoned') NOT NULL DEFAULT 'InProgress',
    PRIMARY KEY (`id`),
    INDEX `idx_exam_attempts_user` (`user_id`),
    INDEX `idx_exam_attempts_version` (`exam_version_id`),
    INDEX `idx_exam_attempts_status` (`status`),
    CONSTRAINT `fk_exam_attempts_user`
        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_exam_attempts_version`
        FOREIGN KEY (`exam_version_id`) REFERENCES `exam_versions` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `attempt_answers` (
    `id`                  BIGINT   NOT NULL AUTO_INCREMENT,
    `attempt_id`          BIGINT   NOT NULL,
    `question_id`         BIGINT   NOT NULL,
    `selected_option_ids` JSON     NULL,
    `is_correct`          BOOLEAN  NULL,
    `answered_at`         DATETIME NULL,
    PRIMARY KEY (`id`),
    INDEX `idx_attempt_answers_attempt` (`attempt_id`),
    INDEX `idx_attempt_answers_question` (`question_id`),
    CONSTRAINT `fk_attempt_answers_attempt`
        FOREIGN KEY (`attempt_id`) REFERENCES `exam_attempts` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_attempt_answers_question`
        FOREIGN KEY (`question_id`) REFERENCES `questions` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `favorites` (
    `id`          BIGINT   NOT NULL AUTO_INCREMENT,
    `user_id`     BIGINT   NOT NULL,
    `question_id` BIGINT   NOT NULL,
    `created_at`  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_favorites_user_question` (`user_id`, `question_id`),
    INDEX `idx_favorites_question` (`question_id`),
    CONSTRAINT `fk_favorites_user`
        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_favorites_question`
        FOREIGN KEY (`question_id`) REFERENCES `questions` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `wrong_answer_notebook` (
    `id`                   BIGINT   NOT NULL AUTO_INCREMENT,
    `user_id`              BIGINT   NOT NULL,
    `question_id`          BIGINT   NOT NULL,
    `wrong_count`          INT      NOT NULL DEFAULT 1,
    `last_wrong_at`        DATETIME NULL,
    `next_review_at`       DATETIME NULL,
    `review_interval_days` INT      NOT NULL DEFAULT 1,
    `created_at`           DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_wrong_notebook_user_question` (`user_id`, `question_id`),
    INDEX `idx_wrong_notebook_next_review` (`next_review_at`),
    CONSTRAINT `fk_wrong_notebook_user`
        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `fk_wrong_notebook_question`
        FOREIGN KEY (`question_id`) REFERENCES `questions` (`id`)
        ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `analytics_snapshots` (
    `id`            BIGINT NOT NULL AUTO_INCREMENT,
    `user_id`       BIGINT NULL,
    `snapshot_type` ENUM('UserScore','SubjectStats','DifficultyBreakdown','DailyActivity') NOT NULL,
    `snapshot_data` JSON   NOT NULL,
    `snapshot_date` DATE   NOT NULL,
    `created_at`    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    INDEX `idx_analytics_user` (`user_id`),
    INDEX `idx_analytics_type` (`snapshot_type`),
    INDEX `idx_analytics_date` (`snapshot_date`),
    CONSTRAINT `fk_analytics_user`
        FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
        ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
