-- Your SQL goes here
CREATE TABLE IF NOT EXISTS auths(
    id            UUID PRIMARY KEY NOT NULL,
    user_id       UUID             NOT NULL REFERENCES users(id),
    expires_at    TIMESTAMP        NOT NULL,
    created_at    TIMESTAMP        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP        NOT NULL DEFAULT CURRENT_TIMESTAMP
)

