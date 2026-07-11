-- forms: form definitions owned by a client, identified by external_id
CREATE TABLE IF NOT EXISTS forms (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    data         TEXT    NOT NULL DEFAULT '{}',  -- JSON form definition
    is_active    BOOLEAN NOT NULL DEFAULT TRUE,
    client_id    INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    external_id  TEXT    NOT NULL,
    admin_token  TEXT    NOT NULL UNIQUE,
    submit_token TEXT    NOT NULL UNIQUE,
    webhook_url  TEXT,
    created_at   TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at   TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE (client_id, external_id)
);
