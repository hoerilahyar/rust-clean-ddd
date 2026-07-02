CREATE TABLE refresh_tokens (

    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,

    user_id BIGINT UNSIGNED NOT NULL,

    device_id VARCHAR(255) NOT NULL,

    ip_address VARCHAR(50),

    token TEXT NOT NULL,

    expired_at TIMESTAMP NOT NULL,

    revoked_at TIMESTAMP NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    INDEX idx_user(user_id),

    INDEX idx_expired(expired_at),

    FOREIGN KEY(user_id)
        REFERENCES users(id)

);