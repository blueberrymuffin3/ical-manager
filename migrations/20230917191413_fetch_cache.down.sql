CREATE TABLE SourceFileTemp (
    id INTEGER NOT NULL PRIMARY KEY REFERENCES Feed(id),
    contents BLOB NOT NULL
) STRICT;

INSERT INTO SourceFileTemp
  SELECT s.id, c.data from SourceFile s
  INNER JOIN FetchCache c ON c.id = s.id;

PRAGMA foreign_keys=off;
DROP TABLE SourceFile;
ALTER TABLE SourceFileTemp RENAME TO SourceFile;
PRAGMA foreign_keys=on;

DROP TABLE FetchCache;
