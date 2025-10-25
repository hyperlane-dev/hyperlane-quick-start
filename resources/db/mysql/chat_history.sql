CREATE TABLE `chat_history` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `session_id` VARCHAR(255) NOT NULL,
    `sender_name` VARCHAR(255) NOT NULL,
    `sender_type` VARCHAR(50) NOT NULL,
    `message_type` VARCHAR(50) NOT NULL,
    `content` TEXT NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `idx_session_id` (`session_id`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci