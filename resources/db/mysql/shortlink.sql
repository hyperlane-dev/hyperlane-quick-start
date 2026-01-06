CREATE TABLE `shortlink` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `url` TEXT NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_shortlink_url` (`url`(255)),
    KEY `idx_shortlink_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;