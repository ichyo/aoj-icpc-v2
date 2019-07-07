-- Your SQL goes here
CREATE TYPE SOURCETYPE AS ENUM ('domestic', 'regional', 'jag_domestic', 'jag_regional', 'others');

ALTER TABLE problems
    ADD COLUMN year SMALLINT NOT NULL DEFAULT 2000,
    ADD COLUMN source_type SOURCETYPE NOT NULL DEFAULT 'others';
