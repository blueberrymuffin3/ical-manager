INSERT INTO Feed(id, name, link_code) VALUES (1, 'Hello World', '446fc76e7cf24f8a819e600528860329');
INSERT INTO SourceFile(id, contents) VALUES (1, X'48656c6c6f200d576f726c642046696c6520436f6e74656e74730d0a');

INSERT INTO Feed(id, name, link_code) VALUES (2, 'Funny Holidays', 'd9d03e2fce5244768391cad3a9b6cb9a');
INSERT INTO SourceHTTP(id, link) VALUES (2, 'https://www.webcal.guru/en-US/download_calendar?calendar_instance_id=142');

INSERT INTO Feed(id, name, link_code)  VALUES(3, 'US Federal Holidays', 'ad7c4acf21b040269c10a154eb837a91');
INSERT INTO SourceHTTP(id, link) VALUES(3, 'https://www.webcal.guru/en-US/download_calendar?calendar_instance_id=41');
