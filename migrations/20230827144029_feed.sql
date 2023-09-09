CREATE TABLE Feed (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    link_code TEXT NOT NULL UNIQUE,
    CONSTRAINT link_code_lower CHECK (link_code = lower(link_code))
) STRICT;

CREATE TABLE SourceFile (
    id INTEGER NOT NULL PRIMARY KEY REFERENCES Feed(id),
    contents BLOB NOT NULL
) STRICT;

CREATE TABLE SourceHTTP (
    id INTEGER NOT NULL PRIMARY KEY REFERENCES Feed(id),
    link TEXT NOT NULL
) STRICT;

CREATE TABLE FilterRemoveCarriageReturn (
    id INTEGER NOT NULL PRIMARY KEY REFERENCES Feed(id)
) STRICT;

INSERT INTO Feed(id, name, link_code) VALUES (1, 'Hello World', '446fc76e7cf24f8a819e600528860329');
INSERT INTO SourceFile(id, contents) VALUES (1, X'48656c6c6f200d576f726c642046696c6520436f6e74656e74730d0a');
INSERT INTO FilterRemoveCarriageReturn(id) VALUES (1);

INSERT INTO Feed(id, name, link_code) VALUES (2, 'Hatsune Miku', 'd9d03e2fce5244768391cad3a9b6cb9a');
INSERT INTO SourceHTTP(id, link) VALUES (2, 'https://example.com/feed.ical');
