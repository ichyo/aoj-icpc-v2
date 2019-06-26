-- Your SQL goes here
CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    source VARCHAR NOT NULL,
    point INT NOT NULL,
    url VARCHAR NOT NULL
)
