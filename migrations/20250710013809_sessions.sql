CREATE TABLE IF NOT EXISTS sessions
(
    id          TEXT PRIMARY KEY,
    exp         DATETIME    NOT NULL,
    user_id     TEXT        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    user_agent  TEXT        NOT NULL DEFAULT ''
);