#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use crate::db::Pstatus;

    aoj_problems (problem_id) {
        problem_id -> Int4,
        aoj_id -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::Pstatus;

    aoj_solutions (aoj_user_id, aoj_problem_id) {
        aoj_user_id -> Int4,
        aoj_problem_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::Pstatus;

    aoj_users (id) {
        id -> Int4,
        aoj_id -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::Pstatus;

    problems (id) {
        id -> Int4,
        title -> Varchar,
        source -> Varchar,
        point -> Int4,
        url -> Varchar,
        status -> Pstatus,
    }
}

joinable!(aoj_problems -> problems (problem_id));
joinable!(aoj_solutions -> aoj_problems (aoj_problem_id));
joinable!(aoj_solutions -> aoj_users (aoj_user_id));

allow_tables_to_appear_in_same_query!(
    aoj_problems,
    aoj_solutions,
    aoj_users,
    problems,
);
