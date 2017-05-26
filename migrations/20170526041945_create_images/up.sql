-- Your SQL goes here
CREATE TABLE IF NOT EXISTS images (
    id      INT             PRIMARY KEY NOT NULL,
    name    VARCHAR(64)                 NOT NULL,
    uri     VARCHAR(255)                NOT NULL,
    mime    VARCHAR(32)                 NOT NULL,
    issue   INT                         NOT NULL,
    
    FOREIGN KEY(issue) REFERENCES issues(id)
)