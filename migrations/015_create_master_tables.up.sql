DROP TABLE IF EXISTS master_data_items;
DROP TABLE IF EXISTS master_data_groups;

CREATE TABLE master_data_groups (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,

    code VARCHAR(50) NOT NULL,
    name VARCHAR(150) NOT NULL,
    description VARCHAR(255) NULL,
    is_hierarchical BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME NULL,
    delete_marker CHAR(36) NULL,

    code_unique VARCHAR(87) AS (
        CONCAT(code, '|', COALESCE(delete_marker, ''))
    ) STORED,
    UNIQUE KEY uq_master_data_groups_code (code_unique)
);

CREATE TABLE master_data_items (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,

    group_id BIGINT UNSIGNED NOT NULL,
    parent_id BIGINT UNSIGNED NULL,

    code VARCHAR(100) NOT NULL,
    name VARCHAR(255) NOT NULL,
    metadata JSON NULL,
    sort_order INT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME NULL,
    delete_marker CHAR(36) NULL,

    group_code_unique VARCHAR(137) AS (
        CONCAT(group_id, '|', code, '|', COALESCE(delete_marker, ''))
    ) STORED,
    UNIQUE KEY uq_master_data_items_group_code (group_code_unique),

    CONSTRAINT fk_master_data_items_group
        FOREIGN KEY (group_id) REFERENCES master_data_groups (id),
    CONSTRAINT fk_master_data_items_parent
        FOREIGN KEY (parent_id) REFERENCES master_data_items (id)
);

CREATE INDEX idx_master_data_items_group ON master_data_items (group_id);
CREATE INDEX idx_master_data_items_parent ON master_data_items (parent_id);

INSERT INTO permissions (resource, action, code, name, description, is_active) VALUES
('master_data_group', 'create', 'MASTER_DATA_GROUP_CREATE', 'Create Master Data Group', 'Create a new master data group', TRUE),
('master_data_group', 'read', 'MASTER_DATA_GROUP_READ', 'Read Master Data Group', 'View master data groups', TRUE),
('master_data_group', 'update', 'MASTER_DATA_GROUP_UPDATE', 'Update Master Data Group', 'Update a master data group', TRUE),
('master_data_group', 'delete', 'MASTER_DATA_GROUP_DELETE', 'Delete Master Data Group', 'Delete a master data group', TRUE),
('master_data_item', 'create', 'MASTER_DATA_ITEM_CREATE', 'Create Master Data Item', 'Create a new master data item', TRUE),
('master_data_item', 'read', 'MASTER_DATA_ITEM_READ', 'Read Master Data Item', 'View master data items', TRUE),
('master_data_item', 'update', 'MASTER_DATA_ITEM_UPDATE', 'Update Master Data Item', 'Update a master data item', TRUE),
('master_data_item', 'delete', 'MASTER_DATA_ITEM_DELETE', 'Delete Master Data Item', 'Delete a master data item', TRUE);

INSERT INTO role_permissions (role_id, permission_id)
SELECT (SELECT id FROM roles WHERE code = 'ADMIN'), id
FROM permissions
WHERE code IN (
    'MASTER_DATA_GROUP_CREATE', 'MASTER_DATA_GROUP_READ',
    'MASTER_DATA_GROUP_UPDATE', 'MASTER_DATA_GROUP_DELETE',
    'MASTER_DATA_ITEM_CREATE', 'MASTER_DATA_ITEM_READ',
    'MASTER_DATA_ITEM_UPDATE', 'MASTER_DATA_ITEM_DELETE'
);

INSERT INTO menus (parent_id, name, icon, path, sort_order, is_active)
VALUES (NULL, 'Master Data', 'database', '/master-data', 6, TRUE);

INSERT INTO menu_permissions (menu_id, permission_id)
SELECT
    (SELECT id FROM menus WHERE name = 'Master Data'),
    p.id
FROM permissions p
WHERE p.code IN (
    'MASTER_DATA_GROUP_READ', 'MASTER_DATA_GROUP_CREATE', 'MASTER_DATA_GROUP_UPDATE', 'MASTER_DATA_GROUP_DELETE',
    'MASTER_DATA_ITEM_READ', 'MASTER_DATA_ITEM_CREATE', 'MASTER_DATA_ITEM_UPDATE', 'MASTER_DATA_ITEM_DELETE'
);

INSERT INTO master_data_groups (code, name, description, is_hierarchical, is_active, created_at, updated_at)
VALUES
    ('gender', 'Gender', 'Jenis kelamin', FALSE, TRUE, UTC_TIMESTAMP(), UTC_TIMESTAMP()),
    ('country', 'Country', 'Daftar negara', FALSE, TRUE, UTC_TIMESTAMP(), UTC_TIMESTAMP());

INSERT INTO master_data_items (group_id, parent_id, code, name, metadata, sort_order, is_active, created_at, updated_at)
SELECT id, NULL, 'M', 'Male', NULL, 1, TRUE, UTC_TIMESTAMP(), UTC_TIMESTAMP() FROM master_data_groups WHERE code = 'gender'
UNION ALL
SELECT id, NULL, 'F', 'Female', NULL, 2, TRUE, UTC_TIMESTAMP(), UTC_TIMESTAMP() FROM master_data_groups WHERE code = 'gender'
UNION ALL
SELECT id, NULL, 'ID', 'Indonesia', JSON_OBJECT('dial_code', '+62'), 1, TRUE, UTC_TIMESTAMP(), UTC_TIMESTAMP() FROM master_data_groups WHERE code = 'country'
UNION ALL
SELECT id, NULL, 'SG', 'Singapore', JSON_OBJECT('dial_code', '+65'), 2, TRUE, UTC_TIMESTAMP(), UTC_TIMESTAMP() FROM master_data_groups WHERE code = 'country';
