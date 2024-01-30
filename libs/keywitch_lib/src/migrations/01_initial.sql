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
