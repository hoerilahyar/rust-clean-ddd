CREATE TABLE menu_permissions (

    menu_id BIGINT UNSIGNED,

    permission_id BIGINT UNSIGNED,

    PRIMARY KEY(menu_id,permission_id),

    FOREIGN KEY(menu_id)
        REFERENCES menus(id),

    FOREIGN KEY(permission_id)
        REFERENCES permissions(id)

);