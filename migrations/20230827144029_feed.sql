CREATE TABLE Feed (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    link_code TEXT NOT NULL UNIQUE,
    filters BLOB NOT NULL,
    source_link TEXT,
    ttl_seconds INTEGER,
    last_update INTEGER,
    CONSTRAINT link_code_lower CHECK (link_code = lower(link_code))
) strict;

-- x'5b5d' is JSON []
INSERT INTO Feed(name, link_code, filters, source_link, ttl_seconds) VALUES('Hello World', '446fc76e7cf24f8a819e600528860329', X'5b5d', 'https://example.com/feed.ical', 21600);
INSERT INTO Feed(name, link_code, filters) VALUES('Hatsune Miku', 'd9d03e2fce5244768391cad3a9b6cb9a', X'5b5d');
