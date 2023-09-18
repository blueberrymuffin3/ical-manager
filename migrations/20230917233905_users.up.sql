CREATE TABLE User (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT,
  icon TEXT
) STRICT;

CREATE TABLE UserOAuthLink (
  issuer TEXT NOT NULL,
  subject TEXT NOT NULL,
  id INTEGER NOT NULL REFERENCES User(id),
  PRIMARY KEY(issuer, subject),
  UNIQUE(id, issuer)
);

ALTER TABLE Feed ADD COLUMN user_id INTEGER REFERENCES User(id);
