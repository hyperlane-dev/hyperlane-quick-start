CREATE TABLE `shortlink` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `url` TEXT NOT NULL,
    `user_cookie` VARCHAR(255),
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `idx_shortlink_user_cookie` (`user_cookie`),
    KEY `idx_shortlink_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;