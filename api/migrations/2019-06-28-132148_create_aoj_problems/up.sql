-- Your SQL goes here
CREATE TABLE aoj_problems (
    problem_id INTEGER PRIMARY KEY REFERENCES problems(id),
    aoj_id VARCHAR NOT NULL UNIQUE
)