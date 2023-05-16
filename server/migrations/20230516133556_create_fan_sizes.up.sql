CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE fan_sizes (
  fan_size_id VARCHAR(255) NOT NULL PRIMARY KEY,
  fan_series_id VARCHAR(255) NOT NULL,
  diameter FLOAT NOT NULL,
  outlet_area FLOAT NOT NULL,
  CONSTRAINT fk_fan_series_id FOREIGN KEY (fan_series_id) REFERENCES fan_serieses(fan_series_id)
);