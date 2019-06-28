
use crate::schema::aoj_problems;
use crate::schema::problems;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::Connection as _;

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

#[derive(Insertable)]
#[table_name = "aoj_problems"]
pub struct NewAojProblem {
    pub problem_id: i32,
    pub aoj_id: String,
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

pub fn get_problems(connection: &Connection) -> Vec<Problem> {
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