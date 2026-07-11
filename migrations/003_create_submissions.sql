CREATE TABLE IF NOT EXISTS submissions (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    form_id      INTEGER NOT NULL REFERENCES forms(id) ON DELETE CASCADE,
    values       TEXT    NOT NULL DEFAULT '{}',  -- JSON submitted field values
    submitted_at TEXT    NOT NULL DEFAULT (datetime('now'))
);
