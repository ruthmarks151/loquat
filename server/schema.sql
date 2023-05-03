CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
DROP TABLE IF EXISTS fan_sizes CASCADE;
DROP TABLE IF EXISTS fan_serieses CASCADE;

CREATE TABLE fan_serieses (
  id VARCHAR(255) NOT NULL PRIMARY KEY,
  fan_type VARCHAR(64) NOT NULL,

  CONSTRAINT chk_fan_type CHECK (fan_type IN ('centrifugal', 'mixed_flow', 'axial', 'induced_flow'))
);

CREATE TABLE fan_sizes (
  id VARCHAR(255) NOT NULL PRIMARY KEY,
  fan_series_id VARCHAR(255) NOT NULL,
  diameter FLOAT NOT NULL,

  CONSTRAINT fk_fan_series_id FOREIGN KEY (fan_series_id) REFERENCES fan_serieses(id)

);

INSERT INTO fan_serieses (id, fan_type)
  VALUES ('SKYPLUME G2-ELLV DMF', 'centrifugal');

INSERT INTO fan_serieses (id, fan_type)
  VALUES ('SKYPLUME G1-ELLV DMF', 'mixed_flow');

INSERT INTO fan_sizes (id, fan_series_id, diameter)
  VALUES ('SKYPLUME G1-ELLV DMF-150', 'SKYPLUME G1-ELLV DMF', 18.25);

INSERT INTO fan_sizes (id, fan_series_id, diameter)
  VALUES ('SKYPLUME G1-ELLV DMF-250', 'SKYPLUME G1-ELLV DMF', 25.0);