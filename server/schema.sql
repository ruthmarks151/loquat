CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
DROP TABLE IF EXISTS fans;

CREATE TABLE fans (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  name TEXT NOT NULL
);

INSERT INTO fans (id, name)
  VALUES ('a3e49b62-cba8-48ce-9cea-d850cb6bd8a3', 'Big Fan');

INSERT INTO fans (name)
  VALUES ('Middle Fan');

INSERT INTO fans (name)
  VALUES ('Baby Fan');
