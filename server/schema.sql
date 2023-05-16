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
  outlet_area FLOAT NOT NULL,
  CONSTRAINT fk_fan_series_id FOREIGN KEY (fan_series_id) REFERENCES fan_serieses(id)

);