-- clients: applications that authenticate via API key to create/manage forms
CREATE TABLE IF NOT EXISTS clients (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL UNIQUE,
    api_key_hash TEXT  NOT NULL,
    is_active  BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT    NOT NULL DEFAULT (datetime('now'))
);
