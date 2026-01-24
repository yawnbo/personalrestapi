-- Create oidc_identities table for linking external auth providers to users
CREATE TABLE IF NOT EXISTS oidc_identities
(
    id               TEXT NOT NULL PRIMARY KEY,
    user_id          TEXT NOT NULL,
    provider         TEXT NOT NULL,
    provider_user_id TEXT NOT NULL,
    email            TEXT,
    created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Unique constraint: one identity per provider per provider_user_id
CREATE UNIQUE INDEX IF NOT EXISTS oidc_identities_provider_user_idx ON oidc_identities (provider, provider_user_id);

-- Index for looking up identities by user
CREATE INDEX IF NOT EXISTS oidc_identities_user_id_idx ON oidc_identities (user_id);
