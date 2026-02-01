CREATE TABLE IF NOT EXISTS cicd_job (
    id INT AUTO_INCREMENT PRIMARY KEY,
    run_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    runner VARCHAR(255),
    started_at DATETIME,
    completed_at DATETIME,
    duration_ms INT DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_run_id (run_id),
    INDEX idx_status (status),
    FOREIGN KEY (run_id) REFERENCES cicd_run (id) ON DELETE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;