-- Your SQL goes here
CREATE TABLE IF NOT EXISTS auths(
    id            UUID PRIMARY KEY NOT NULL,
    user_id       UUID             NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at    TIMESTAMP        NOT NULL,
    created_at    TIMESTAMP        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP        NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TRIGGER update_last_updated_at BEFORE UPDATE ON auths FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at();
