-- Your SQL goes here
CREATE TABLE aoj_solutions (
    aoj_problem_id INTEGER NOT NULL REFERENCES aoj_problems(problem_id),
    aoj_user_id INTEGER NOT NULL REFERENCES aoj_users(id),
    PRIMARY KEY (aoj_problem_id, aoj_user_id)
)