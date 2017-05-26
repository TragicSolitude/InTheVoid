-- Your SQL goes here
CREATE TABLE IF NOT EXISTS issues (
    id          INTEGER PRIMARY KEY,
    summary     TEXT,
    description TEXT,
    priority    INT
);