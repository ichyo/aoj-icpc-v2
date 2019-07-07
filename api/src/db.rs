use crate::schema::aoj_problems;
use crate::schema::aoj_solutions;
use crate::schema::aoj_users;
use crate::schema::problems;
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use diesel::sql_types::{BigInt, Integer};
use diesel::Connection as _;
use diesel_derive_enum::DbEnum;
use std::collections::HashMap;
use std::string::ToString;

#[derive(DbEnum, Debug)]
#[DieselType = "Pstatus"]
pub enum ProblemStatus {
    Pending,
    Active,
    Hidden,
}

impl ToString for ProblemStatus {
    fn to_string(&self) -> String {
        match self {
            ProblemStatus::Pending => "pending".to_string(),
            ProblemStatus::Active => "active".to_string(),
            ProblemStatus::Hidden => "hidden".to_string(),
        }
    }
}

#[derive(DbEnum, Debug)]
#[DieselType = "Sourcetype"]
pub enum SourceType {
    Domestic,
    Regional,
    JagDomestic,
    JagRegional,
    Others,
}

impl ToString for SourceType {
    fn to_string(&self) -> String {
        match &self {
            SourceType::Domestic => "domestic".to_string(),
            SourceType::Regional => "regional".to_string(),
            SourceType::JagDomestic => "jag_domestic".to_string(),
            SourceType::JagRegional => "jag_regional".to_string(),
            SourceType::Others => "other".to_string(),
        }
    }
}

#[derive(Insertable, Queryable, AsChangeset, Debug)]
#[table_name = "problems"]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub source: String,
    pub point: i32,
    pub url: String,
    pub status: ProblemStatus,
    pub year: i16,
    pub source_type: SourceType,
}

#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "aoj_problems"]
pub struct AojProblem {
    pub problem_id: i32,
    pub aoj_id: String,
}

#[derive(Insertable, Debug)]
#[table_name = "aoj_solutions"]
pub struct NewSolution {
    pub aoj_user_id: i32,
    pub aoj_problem_id: i32,
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "aoj_users"]
pub struct AojUser {
    pub id: i32,
    pub aoj_id: String,
}

#[derive(Insertable, Debug, Eq, PartialEq, Hash)]
#[table_name = "aoj_users"]
pub struct NewAojUser {
    pub aoj_id: String,
}

#[derive(Queryable, Associations, Identifiable, Insertable, Debug)]
#[table_name = "aoj_solutions"]
#[belongs_to(AojUser)]
#[primary_key(aoj_user_id, aoj_problem_id)]
pub struct AojSolution {
    pub aoj_user_id: i32,
    pub aoj_problem_id: i32,
    pub submission_time: DateTime<Utc>,
}

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn create_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool")
}

pub fn establish_connection(database_url: &str) -> Connection {
    PgConnection::establish(database_url).expect("Failed to connect DB")
}

pub fn get_all_problems(connection: &Connection) -> Vec<Problem> {
    use crate::schema::problems::dsl::*;
    problems
        .order(id.asc())
        .load::<Problem>(connection)
        .expect("Failed to query problems")
}

pub fn update_problems(
    connection: &Connection,
    new_problems: &[Problem],
    aoj_problems: &[AojProblem],
) {
    for problem in new_problems {
        diesel::insert_into(problems::table)
            .values(problem)
            .on_conflict(problems::dsl::id)
            .do_update()
            .set(problem)
            .execute(connection)
            .expect("Failed to save new problems");
    }
    for problem in aoj_problems {
        diesel::insert_into(aoj_problems::table)
            .values(problem)
            .on_conflict(aoj_problems::dsl::problem_id)
            .do_update()
            .set(problem)
            .execute(connection)
            .expect("Failed to save new aoj_problems");
    }
}

pub fn get_aoj_users_by_aoj_ids(connection: &Connection, aoj_ids: &[String]) -> Vec<AojUser> {
    use crate::schema::aoj_users::dsl::*;
    aoj_users
        .filter(aoj_id.eq_any(aoj_ids))
        .load::<AojUser>(connection)
        .expect("Failed to load aoj users")
}

pub fn insert_aoj_users(connection: &Connection, users: &[NewAojUser]) -> Vec<AojUser> {
    diesel::insert_into(aoj_users::table)
        .values(users)
        .on_conflict_do_nothing()
        .get_results(connection)
        .expect("Failed to insert aoj users")
}

pub fn get_all_aoj_problems(connection: &Connection) -> Vec<AojProblem> {
    use crate::schema::aoj_problems::dsl::*;
    aoj_problems
        .load::<AojProblem>(connection)
        .expect("Failed to load aoj problems")
}

pub fn insert_aoj_solutions(connection: &Connection, solutions: &[AojSolution]) {
    for solutions in solutions.chunks(10000) {
        diesel::insert_into(aoj_solutions::table)
            .values(solutions)
            .on_conflict_do_nothing()
            .execute(connection)
            .expect("Failed to insert aoj solutions");
    }
}

pub fn get_aoj_solutions_by_aoj_user<'a>(
    connection: &'_ Connection,
    users: &'a [AojUser],
) -> Vec<(&'a AojUser, Vec<AojSolution>)> {
    let solutions = AojSolution::belonging_to(users)
        .load::<AojSolution>(connection)
        .expect("Failed to load solutions")
        .grouped_by(users);
    users.iter().zip(solutions).collect::<Vec<_>>()
}

pub fn get_number_of_solutions_by_problems(connection: &Connection) -> HashMap<i32, i64> {
    #[derive(QueryableByName)]
    struct Count {
        #[sql_type = "Integer"]
        aoj_problem_id: i32,
        #[sql_type = "BigInt"]
        count: i64,
    };
    let count: Vec<Count> = sql_query(
        "SELECT aoj_problem_id, COUNT(*) as count FROM aoj_solutions GROUP BY aoj_problem_id",
    )
    .load(connection)
    .expect("Query failed");
    count
        .iter()
        .map(|c| (c.aoj_problem_id, c.count))
        .collect::<HashMap<_, _>>()
}
