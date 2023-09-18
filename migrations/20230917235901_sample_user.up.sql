INSERT INTO User(id) VALUES (1);
INSERT INTO UserOAuthLink(id, issuer, subject) VALUES (1, 'https://accounts.google.com', '117845186291315157803');
UPDATE Feed SET user_id = 1;
