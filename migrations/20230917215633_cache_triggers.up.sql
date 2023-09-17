CREATE TRIGGER SourceHttpDeleteFetchCachePurge
AFTER DELETE ON SourceHttp
BEGIN
  DELETE FROM FetchCache WHERE id = old.id;
END;

CREATE TRIGGER SourceHttpUpdateFetchCachePurge
AFTER UPDATE ON SourceHttp
WHEN new.link <> old.link
BEGIN
  DELETE FROM FetchCache WHERE id = new.id;
END;

CREATE TRIGGER SourceFileDeleteFetchCachePurge
AFTER DELETE ON SourceFile
BEGIN
  DELETE FROM FetchCache WHERE id = old.id;
END;
