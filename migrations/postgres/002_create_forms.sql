CREATE TABLE IF NOT EXISTS forms (
    id           BIGSERIAL PRIMARY KEY,
    client_id    BIGINT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    external_id  TEXT NOT NULL,
    data         TEXT NOT NULL DEFAULT '{}',
    admin_token  TEXT NOT NULL UNIQUE,
    submit_token TEXT NOT NULL UNIQUE,
    webhook_url  TEXT,
    is_active    BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (client_id, external_id)
);
