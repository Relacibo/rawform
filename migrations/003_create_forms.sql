CREATE TABLE IF NOT EXISTS forms (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id     INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    external_id   TEXT    NOT NULL,
    definition_id INTEGER NOT NULL REFERENCES form_definitions(id),
    admin_token   TEXT    NOT NULL UNIQUE,
    submit_token  TEXT    NOT NULL UNIQUE,
    webhook_url   TEXT,
    is_active     BOOLEAN NOT NULL DEFAULT TRUE,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE (client_id, external_id)
);
