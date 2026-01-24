-- Add auth_provider column to users table
-- NULL = password-based user, 'google'/'github'/'apple' = OIDC user
ALTER TABLE users ADD COLUMN auth_provider TEXT DEFAULT NULL;
