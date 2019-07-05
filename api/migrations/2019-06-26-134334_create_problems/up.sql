-- Your SQL goes here
CREATE TYPE PSTATUS AS ENUM ('pending', 'active', 'hidden');

CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    source VARCHAR NOT NULL,
    point INT NOT NULL,
    url VARCHAR NOT NULL,
    status PSTATUS NOT NULL
)
