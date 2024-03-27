CREATE TABLE keys
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    pinned      INTEGER DEFAULT FALSE,
    target_size INTEGER             NOT NULL,
    revision    INTEGER             NOT NULL,
    charset     TEXT                NOT NULL,
    domain      TEXT COLLATE NOCASE NOT NULL,
    username    TEXT COLLATE NOCASE NOT NULL,
    notes       TEXT,
    created_at  INTEGER             NOT NULL,
    custom_icon TEXT,
    version     TEXT                NOT NULL
);

CREATE INDEX idx_keys_user_name ON keys (username);
CREATE INDEX idx_keys_pinned ON keys (pinned);
CREATE INDEX idx_keys_domain ON keys (domain);
CREATE INDEX idx_keys_created_at ON keys (created_at DESC);

CREATE TABLE tags
(
    name   TEXT    NOT NULL COLLATE NOCASE,
    key_id INTEGER NOT NULL,
    FOREIGN KEY (key_id) REFERENCES keys (id),
    PRIMARY KEY (name, key_id)
);

CREATE INDEX idx_tags_name ON tags (name COLLATE NOCASE);
CREATE INDEX idx_key_id ON tags (key_id);

CREATE TABLE charsets
(
    name        TEXT PRIMARY KEY NOT NULL COLLATE NOCASE,
    charset     TEXT             NOT NULL,
    description TEXT
);

CREATE VIEW vw_tag_list (tags, key_id) AS
SELECT json_group_array(name) as tags, key_id
FROM tags
GROUP BY key_id;

INSERT INTO charsets (name, charset, description)
VALUES ('Alpha Numeric', 'a..zA..Z0..9', NULL),
       ('Numeric', '0..9', NULL),
       ('Alpha', 'a..zA..Z', NULL);

CREATE
VIRTUAL TABLE search_index USING fts5
(
    domain,
    username,
    notes,
    tags
);
INSERT INTO search_index(search_index, rank)
VALUES ('rank', 'bm25(5.0, 7.0, 1.0, 10.0)');
