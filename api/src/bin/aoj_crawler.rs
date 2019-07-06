use aoj_client::solution::FindAllRequest;
use aoj_client::solution::FindByProblemIdRequest;
use aoj_client::Client;
use failure::Error;

use aoj_icpc::db;
use log::debug;
use log::info;
use std::collections::{HashMap, HashSet};
use chrono::DateTime;
use chrono::Utc;

struct AojSolution {
    user_aoj_id: String,
    problem_aoj_id: String,
    submission_time: DateTime<Utc>,
}

fn from_status(size: u32) -> Result<Vec<AojSolution>, Error> {
    let client = Client::default();

    let solutions = client
        .solution_client()
        .find_all(FindAllRequest::default().set_page(0).set_size(size))?;

    let solutions = solutions
        .into_iter()
        .map(|s| AojSolution {
            user_aoj_id: s.user_id,
            problem_aoj_id: s.problem_id,
            submission_time: s.submission_date,
        })
        .collect::<Vec<_>>();

    Ok(solutions)
}

fn from_problems(connection: &db::Connection) -> Result<Vec<AojSolution>, Error> {
    let client = Client::default();
    let problems = db::get_all_aoj_problems(&connection);

    let mut solutions = Vec::new();
    for p in problems.iter() {
        info!("Fetching solutions for {:?}", p);
        let mut page = 0;
        loop {
            debug!("{:?}: page = {}", p, page);
            let ss = client.solution_client().find_by_problem_id(
                FindByProblemIdRequest::new(p.aoj_id.to_string())
                    .set_page(page)
                    .set_size(100),
            )?;
            if ss.is_empty() {
                break;
            }
            let ss = ss
                .into_iter()
                .map(|s| AojSolution {
                    user_aoj_id: s.user_id,
                    problem_aoj_id: s.problem_id,
                    submission_time: s.submission_date,
                })
                .collect::<Vec<_>>();

            solutions.extend(ss);
            page += 1;

            std::thread::sleep(std::time::Duration::from_millis(800));
        }
    }

    Ok(solutions)
}

fn insert_solutions(connection: db::Connection, solutions: &[AojSolution]) -> Result<(), Error> {
    let problems = db::get_all_aoj_problems(&connection);
    let problems_by_id = problems
        .iter()
        .map(|p| (p.aoj_id.to_string(), p))
        .collect::<HashMap<_, _>>();

    let users = solutions
        .iter()
        .filter_map(|s| match problems_by_id.get(&s.problem_aoj_id) {
            Some(_) => Some(db::NewAojUser {
                aoj_id: s.user_aoj_id.to_string(),
            }),
            None => None,
        })
        .collect::<HashSet<_>>();
    let users = users.into_iter().collect::<Vec<_>>();

    info!("Inserting {} users", users.len());
    debug!("{:?}", users);
    db::insert_aoj_users(&connection, &users);

    let users = db::get_aoj_users_by_aoj_ids(
        &connection,
        &users.into_iter().map(|u| u.aoj_id).collect::<Vec<_>>(),
    );
    let users_by_id = users
        .iter()
        .map(|u| (u.aoj_id.to_string(), u))
        .collect::<HashMap<_, _>>();

    let solutions = solutions
        .iter()
        .filter_map(|s| {
            match (
                problems_by_id.get(&s.problem_aoj_id),
                users_by_id.get(&s.user_aoj_id),
            ) {
                (Some(p), Some(u)) => Some(db::NewAojSolution {
                    aoj_problem_id: p.problem_id,
                    aoj_user_id: u.id,
                    submission_time: s.submission_time,
                }),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    info!("Inserting {} solutions", solutions.len());
    debug!("{:?}", solutions);
    db::insert_aoj_solutions(&connection, &solutions);

    Ok(())
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init_from_env("AOJICPC_LOG");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let connection = db::establish_connection(&database_url);

    //let solutions = from_status(100)?;
    let solutions = from_problems(&connection)?;
    insert_solutions(connection, &solutions)
}
