DROP TABLE IF EXISTS audit_logs;

CREATE TABLE audit_logs (
    id              BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    actor_id        BIGINT UNSIGNED NULL,
    actor_email     VARCHAR(255)    NULL,
    action          VARCHAR(100)    NOT NULL,
    entity_type     VARCHAR(100)    NULL,
    entity_id       VARCHAR(100)    NULL,
    status          VARCHAR(20)     NOT NULL DEFAULT 'success',
    ip_address      VARCHAR(45)     NULL,
    user_agent      TEXT            NULL,
    metadata        JSON            NULL,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_audit_logs_actor_id ON audit_logs (actor_id);
CREATE INDEX idx_audit_logs_action ON audit_logs (action);
CREATE INDEX idx_audit_logs_entity ON audit_logs (entity_type, entity_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs (created_at DESC);