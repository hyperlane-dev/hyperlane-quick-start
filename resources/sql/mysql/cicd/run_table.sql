CREATE TABLE IF NOT EXISTS cicd_run (
    id INT AUTO_INCREMENT PRIMARY KEY,
    pipeline_id INT NOT NULL,
    run_number INT NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    triggered_by VARCHAR(255),
    commit_hash VARCHAR(64),
    commit_message TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    duration_ms INT DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_pipeline_id (pipeline_id),
    INDEX idx_status (status),
    INDEX idx_run_number (run_number),
    FOREIGN KEY (pipeline_id) REFERENCES cicd_pipeline (id) ON DELETE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;