CREATE TABLE audit_logs (

    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,

    user_id BIGINT UNSIGNED,

    resource VARCHAR(100),

    action VARCHAR(100),

    reference_id BIGINT,

    old_value JSON,

    new_value JSON,

    ip_address VARCHAR(50),

    user_agent TEXT,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP

);