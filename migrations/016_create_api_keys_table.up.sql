DROP TABLE IF EXISTS api_keys;

CREATE TABLE api_keys (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,

    name VARCHAR(150) NOT NULL,
    key_prefix VARCHAR(16) NOT NULL,
    key_hash CHAR(64) NOT NULL,
    permissions JSON NOT NULL,

    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at DATETIME NULL,
    last_used_at DATETIME NULL,
    created_by BIGINT UNSIGNED NULL,

    created_at DATETIME NOT NULL,
    updated_at DATETIME NULL,
    deleted_at DATETIME NULL,
    delete_marker CHAR(36) NULL,

    key_prefix_unique VARCHAR(53) AS (
        CONCAT(key_prefix, '|', COALESCE(delete_marker, ''))
    ) STORED,
    UNIQUE KEY uq_api_keys_prefix (key_prefix_unique),

    CONSTRAINT fk_api_keys_created_by
        FOREIGN KEY (created_by) REFERENCES users (id)
);

CREATE INDEX idx_api_keys_is_active ON api_keys (is_active);

INSERT INTO permissions (resource, action, code, name, description, is_active) VALUES
('api_key', 'create', 'API_KEY_CREATE', 'Create API Key', 'Create a new API key', TRUE),
('api_key', 'read', 'API_KEY_READ', 'Read API Key', 'View API keys', TRUE),
('api_key', 'update', 'API_KEY_UPDATE', 'Update API Key', 'Update an API key', TRUE),
('api_key', 'revoke', 'API_KEY_REVOKE', 'Revoke API Key', 'Activate/deactivate an API key', TRUE),
('api_key', 'delete', 'API_KEY_DELETE', 'Delete API Key', 'Delete an API key', TRUE);

INSERT INTO role_permissions (role_id, permission_id)
SELECT (SELECT id FROM roles WHERE code = 'ADMIN'), id
FROM permissions
WHERE code IN (
    'API_KEY_CREATE', 'API_KEY_READ', 'API_KEY_UPDATE', 'API_KEY_REVOKE', 'API_KEY_DELETE'
);

INSERT INTO menus (parent_id, name, icon, path, sort_order, is_active)
VALUES (NULL, 'API Keys', 'key', '/api-keys', 7, TRUE);

INSERT INTO menu_permissions (menu_id, permission_id)
SELECT
    (SELECT id FROM menus WHERE name = 'API Keys'),
    p.id
FROM permissions p
WHERE p.code IN ('API_KEY_CREATE', 'API_KEY_READ', 'API_KEY_UPDATE', 'API_KEY_REVOKE', 'API_KEY_DELETE');
