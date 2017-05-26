-- Your SQL goes here
CREATE TABLE IF NOT EXISTS issues (
    id          INT     PRIMARY KEY NOT NULL,
    summary     TEXT,
    description TEXT,
    priority    INT
);