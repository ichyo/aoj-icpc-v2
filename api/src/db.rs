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
pub struct NewProblem<'a> {
    pub title: &'a str,
    pub source: &'a str,
    pub point: i32,
    pub url: &'a str,
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

pub fn replace_problems(connection: &Connection, new_problems: &[NewProblem]) -> Vec<Problem> {
    diesel::delete(problems::table)
        .execute(connection)
        .expect("Failed to delete problems");
    diesel::insert_into(problems::table)
        .values(new_problems)
        .get_results(connection)
        .expect("Failed to save new problems")
}