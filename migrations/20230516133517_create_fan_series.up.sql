CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE fan_serieses (
  id VARCHAR(255) NOT NULL PRIMARY KEY,
  fan_type VARCHAR(64) NOT NULL,

  CONSTRAINT chk_fan_type CHECK (fan_type IN ('centrifugal', 'mixed_flow', 'axial', 'induced_flow'))
);