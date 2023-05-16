CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE a1_2010_reports (
  a1_2010_report_id VARCHAR(255) NOT NULL PRIMARY KEY,
  fan_size_id VARCHAR(255) NOT NULL,
  rpm FLOAT NOT NULL,
  -- pub determinations: Vec<A1Standard2010Determination>,
  determinations JSONB NOT NULL,
  CONSTRAINT fk_fan_size_id FOREIGN KEY (fan_size_id) REFERENCES fan_sizes(fan_size_id)
);