DROP TABLE IF EXISTS user_settings;

CREATE TABLE user_settings (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT UNSIGNED NOT NULL,
    setting_key VARCHAR(150) NOT NULL,
    setting_value TEXT NULL,
    data_type ENUM('string', 'number', 'boolean', 'json') NOT NULL DEFAULT 'string',
    description VARCHAR(255) NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NULL,
    deleted_at DATETIME NULL,
    delete_marker CHAR(36) NULL,
    setting_key_unique VARCHAR(200) AS (
        CONCAT(user_id, '|', setting_key, '|', COALESCE(delete_marker, ''))
    ) STORED,
    UNIQUE KEY uq_user_settings_key (setting_key_unique),
    CONSTRAINT fk_user_settings_user
        FOREIGN KEY (user_id) REFERENCES users (id)
        ON DELETE CASCADE,
    INDEX idx_user_settings_user_id (user_id)
);
