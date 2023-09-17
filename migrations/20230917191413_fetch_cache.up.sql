CREATE TABLE FetchCache (
    id INTEGER NOT NULL PRIMARY KEY REFERENCES Feed(id),
    timestamp TEXT NOT NULL,
    data BLOB NOT NULL
) STRICT;

INSERT INTO FetchCache(id, timestamp, data)
  SELECT id, DATETIME('now'), contents
  FROM SourceFile;

CREATE TABLE SourceFileTemp(
  id INTEGER NOT NULL PRIMARY KEY REFERENCES Feed(id)
) STRICT;
INSERT INTO SourceFileTemp SELECT id FROM SourceFile;

PRAGMA foreign_keys=off;
DROP TABLE SourceFile;
ALTER TABLE SourceFileTemp RENAME TO SourceFile;
PRAGMA foreign_keys=on;

