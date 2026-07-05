
ALTER TABLE users
    DROP INDEX idx_username,
    DROP INDEX idx_email;

ALTER TABLE users
    ADD COLUMN delete_marker CHAR(36) NULL DEFAULT NULL AFTER deleted_at;

ALTER TABLE users
    ADD COLUMN username_unique VARCHAR(150)
        GENERATED ALWAYS AS (
            IF(delete_marker IS NULL, username, CONCAT(username, '__', delete_marker))
        ) STORED,
    ADD COLUMN email_unique VARCHAR(300)
        GENERATED ALWAYS AS (
            IF(delete_marker IS NULL, email, CONCAT(email, '__', delete_marker))
        ) STORED;

ALTER TABLE users
    ADD UNIQUE INDEX uq_username_unique (username_unique),
    ADD UNIQUE INDEX uq_email_unique (email_unique);

ALTER TABLE users
    ADD INDEX idx_username_plain (username),
    ADD INDEX idx_email_plain (email);