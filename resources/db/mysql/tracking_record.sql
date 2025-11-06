CREATE TABLE `tracking_record` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `socket_addr` VARCHAR(255) NOT NULL,
    `headers` TEXT NOT NULL,
    `body` TEXT NOT NULL,
    `timestamp` BIGINT NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `idx_socket_addr` (`socket_addr`),
    KEY `idx_timestamp` (`timestamp`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci
