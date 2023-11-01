-- Add migration script here
CREATE TABLE todos (
    id          INTEGER UNIQUE PRIMARY KEY AUTOINCREMENT NOT NULL,
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
);
