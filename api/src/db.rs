use crate::schema::problems;
use crate::schema::aoj_problems;
use crate::schema::aoj_solutions;
use crate::schema::aoj_users;
use diesel::r2d2::{self, ConnectionManager};
use diesel::Connection as _;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub source: String,
    pub point: i32,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "problems"]
pub struct NewProblem {
    pub title: String,
    pub source: String,
    pub point: i32,
    pub url: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "aoj_problems"]
pub struct AojProblem {
    pub problem_id: i32,
    pub aoj_id: String,
}

#[derive(Insertable)]
#[table_name = "aoj_problems"]
pub struct NewAojProblem {
    pub problem_id: i32,
    pub aoj_id: String,
}

#[derive(Insertable)]
#[table_name = "aoj_solutions"]
pub struct NewSolution {
    pub aoj_user_id: i32,
    pub aoj_problem_id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name = "aoj_users"]
pub struct AojUser {
    pub id: i32,
    pub aoj_id: String,
}

#[derive(Insertable)]
#[table_name = "aoj_users"]
pub struct NewAojUser {
    pub aoj_id: String,
}

#[derive(Insertable)]
#[table_name = "aoj_solutions"]
pub struct NewAojSolution {
    pub aoj_problem_id: i32,
    pub aoj_user_id: i32,
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
        .order((point.asc(), source.asc()))
        .load::<Problem>(connection)
        .expect("Failed to query problems")
}

pub fn initialize_problems(connection: &Connection, new_problems: &[NewProblem]) -> Vec<Problem> {
    // required to delete aoj_problems first for foreign key constraint
    diesel::delete(aoj_problems::table)
        .execute(connection)
        .expect("Failed to delete aoj_problems");
    diesel::delete(problems::table)
        .execute(connection)
        .expect("Failed to delete problems");
    diesel::insert_into(problems::table)
        .values(new_problems)
        .get_results(connection)
        .expect("Failed to save new problems")
}

pub fn initialize_aoj_problems(connection: &Connection, new_aoj_problems: &[NewAojProblem]) {
    diesel::delete(aoj_problems::table)
        .execute(connection)
        .expect("Failed to delete aoj_problems");
    diesel::insert_into(aoj_problems::table)
        .values(new_aoj_problems)
        .execute(connection)
        .expect("Failed to save new aoj_problems");
}

pub fn get_aoj_users(connection: &Connection, aoj_ids: &[String]) -> Vec<AojUser> {
    use crate::schema::aoj_users::dsl::*;
    aoj_users
        .filter(aoj_id.eq_any(aoj_ids))
        .load::<AojUser>(connection)
        .expect("Failed to load aoj users")
}

pub fn upsert_aoj_users(connection: &Connection, users: &[NewAojUser]) -> Vec<AojUser> {
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

pub fn upsert_aoj_solutions(connection: &Connection, solutions: &[NewAojSolution]) {
    diesel::insert_into(aoj_solutions::table)
        .values(solutions)
        .on_conflict_do_nothing()
        .execute(connection)
        .expect("Failed to insert aoj solutions");
}