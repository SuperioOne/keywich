CREATE TABLE IF NOT EXISTS _migrations
(
    id   INTEGER PRIMARY KEY,
    name TEXT
);

INSERT INTO _migrations (id, name)
VALUES (1, '01_initial');

CREATE TABLE IF NOT EXISTS keys
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    pinned      INTEGER DEFAULT FALSE,
    target_size INTEGER             NOT NULL,
    revision    INTEGER             NOT NULL,
    charset     TEXT                NOT NULL,
    domain      TEXT COLLATE NOCASE NOT NULL,
    user_name   TEXT COLLATE NOCASE NOT NULL,
    notes       TEXT,
    created_at  INTEGER             NOT NULL,
    custom_icon TEXT,
    version     TEXT                NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_keys_user_name ON keys (user_name);
CREATE INDEX IF NOT EXISTS idx_keys_pinned ON keys (pinned);
CREATE INDEX IF NOT EXISTS idx_keys_domain ON keys (domain);
CREATE INDEX IF NOT EXISTS idx_keys_created_at ON keys (created_at DESC);

CREATE TABLE IF NOT EXISTS tags
(
    name   TEXT    NOT NULL COLLATE NOCASE,
    key_id INTEGER NOT NULL,
    FOREIGN KEY (key_id) REFERENCES keys (id),
    PRIMARY KEY (name, key_id)
);

CREATE INDEX IF NOT EXISTS idx_tags_name ON tags (name COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_key_id ON tags (key_id);

CREATE TABLE IF NOT EXISTS charsets
(
    name        TEXT PRIMARY KEY NOT NULL COLLATE NOCASE,
    charset     TEXT             NOT NULL,
    description TEXT
);

CREATE VIEW IF NOT EXISTS vw_tag_list (tags, key_id) AS
SELECT json_group_array(name) as tags, key_id
FROM tags
GROUP BY key_id;

CREATE TABLE IF NOT EXISTS gc
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    type       INTEGER NOT NULL DEFAULT 0,
    args       TEXT,
    created_at INT     NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_gc_type ON gc (type);
CREATE INDEX IF NOT EXISTS idx_gc_created_at ON gc (created_at DESC);