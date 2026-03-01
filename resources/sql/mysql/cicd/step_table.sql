CREATE TABLE IF NOT EXISTS cicd_step (
    id INT AUTO_INCREMENT PRIMARY KEY,
    job_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    command TEXT,
    status VARCHAR(50) DEFAULT 'pending',
    output LONGTEXT,
    dockerfile LONGTEXT,
    image VARCHAR(512),
    started_at DATETIME,
    completed_at DATETIME,
    duration_ms INT DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_job_id (job_id),
    INDEX idx_status (status),
    FOREIGN KEY (job_id) REFERENCES cicd_job (id) ON DELETE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;