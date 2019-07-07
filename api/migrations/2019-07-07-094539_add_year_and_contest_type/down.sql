-- This file should undo anything in `up.sql`
ALTER TABLE problems
    DROP COLUMN IF EXISTS year,
    DROP COLUMN IF EXISTS source_type;

DROP TYPE IF EXISTS SOURCETYPE;
