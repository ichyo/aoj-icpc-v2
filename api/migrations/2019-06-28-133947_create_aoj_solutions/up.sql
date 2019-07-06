-- Your SQL goes here
CREATE TABLE aoj_solutions (
    aoj_user_id INTEGER NOT NULL REFERENCES aoj_users(id),
    aoj_problem_id INTEGER NOT NULL REFERENCES aoj_problems(problem_id),
    submission_time TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (aoj_user_id, aoj_problem_id)
)
