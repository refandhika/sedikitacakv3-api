CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    level VARCHAR(255) NOT NULL UNIQUE,
    can_modify_user BOOLEAN DEFAULT FALSE NOT NULL,
    can_edit BOOLEAN DEFAULT FALSE NOT NULL,
    can_view BOOLEAN DEFAULT FALSE NOT NULL,
    is_guest BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP
);

INSERT INTO roles (name, level, can_modify_user, can_edit, can_view, is_guest) VALUES ('Administrator', 'administrator', true, true, true, false);
