-- 1. Role: admin
INSERT INTO roles (name, code, description, is_active)
VALUES ('Administrator', 'ADMIN', 'Full access role', TRUE);

-- 2. Permissions (sesuai PermissionCode di src/domain/permission/entity/permission_code.rs)
INSERT INTO permissions (resource, action, code, name, description, is_active) VALUES
('user', 'create', 'USER_CREATE', 'Create User', 'Create a new user', TRUE),
('user', 'read', 'USER_READ', 'Read User', 'View user data', TRUE),
('user', 'update', 'USER_UPDATE', 'Update User', 'Update user data', TRUE),
('user', 'delete', 'USER_DELETE', 'Delete User', 'Delete a user', TRUE),
('role', 'create', 'ROLE_CREATE', 'Create Role', 'Create a new role', TRUE),
('role', 'read', 'ROLE_READ', 'Read Role', 'View role data', TRUE),
('role', 'update', 'ROLE_UPDATE', 'Update Role', 'Update role data', TRUE),
('role', 'delete', 'ROLE_DELETE', 'Delete Role', 'Delete a role', TRUE),
('permission', 'create', 'PERMISSION_CREATE', 'Create Permission', 'Create a new permission', TRUE),
('permission', 'read', 'PERMISSION_READ', 'Read Permission', 'View permission data', TRUE),
('permission', 'update', 'PERMISSION_UPDATE', 'Update Permission', 'Update permission data', TRUE),
('permission', 'delete', 'PERMISSION_DELETE', 'Delete Permission', 'Delete a permission', TRUE),
('role_permission', 'assign', 'ROLE_PERMISSION_ASSIGN', 'Assign Role Permission', 'Assign permission to role', TRUE),
('role_permission', 'revoke', 'ROLE_PERMISSION_REVOKE', 'Revoke Role Permission', 'Revoke permission from role', TRUE),
('role_permission', 'read', 'ROLE_PERMISSION_READ', 'Read Role Permission', 'View role permissions', TRUE),
('user_role', 'assign', 'USER_ROLE_ASSIGN', 'Assign User Role', 'Assign role to user', TRUE),
('user_role', 'revoke', 'USER_ROLE_REVOKE', 'Revoke User Role', 'Revoke role from user', TRUE),
('user_role', 'read', 'USER_ROLE_READ', 'Read User Role', 'View user roles', TRUE);

-- 3. Mapping semua permission ke role admin
INSERT INTO role_permissions (role_id, permission_id)
SELECT (SELECT id FROM roles WHERE code = 'ADMIN'), id FROM permissions;

-- 4. User admin
-- Password: admin123
-- Hash di bawah ini sudah digenerate & diverifikasi valid dengan Argon2id (argon2 crate v0.5, param default)
INSERT INTO users (username, email, password, fullname, is_active)
VALUES (
    'admin',
    'admin@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$8OO3Qpc5VkYwrwrSHqyLIQ$iH8RN50fXoAEETblmd1WW2VdM/rwXNTmSOzFPOqoS6I',
    'Administrator',
    TRUE
);

-- 5. Assign role admin ke user admin
INSERT INTO user_roles (user_id, role_id)
VALUES (
    (SELECT id FROM users WHERE username = 'admin'),
    (SELECT id FROM roles WHERE code = 'ADMIN')
);
