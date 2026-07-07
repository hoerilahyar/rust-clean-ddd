DROP TABLE IF EXISTS system_settings;

CREATE TABLE system_settings (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    setting_key VARCHAR(150) NOT NULL,
    setting_value TEXT NULL,
    data_type ENUM('string', 'number', 'boolean', 'json') NOT NULL DEFAULT 'string',
    description VARCHAR(255) NULL,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NULL,
    deleted_at DATETIME NULL,
    delete_marker CHAR(36) NULL,
    setting_key_unique VARCHAR(151) AS (
        CONCAT(setting_key, '|', COALESCE(delete_marker, ''))
    ) STORED,
    UNIQUE KEY uq_system_settings_key (setting_key_unique)
);