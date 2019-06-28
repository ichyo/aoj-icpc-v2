-- Your SQL goes here
CREATE TABLE aoj_users (
    id SERIAL PRIMARY KEY,
    aoj_id VARCHAR NOT NULL UNIQUE
)