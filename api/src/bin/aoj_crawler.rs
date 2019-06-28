use aoj_client::solution::FindAllRequest;
use aoj_client::Client;
use failure::Error;

use log::debug;
use log::info;
use aoj_icpc::db;
use std::collections::HashMap;


fn main() -> Result<(), Error> {
    env_logger::init_from_env("AOJICPC_LOG");

    let client = Client::default();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let connection = db::establish_connection(&database_url);
    let problems = db::get_all_aoj_problems(&connection);
    let problems_by_id = problems
        .iter()
        .map(|p| (p.aoj_id.to_string(), p))
        .collect::<HashMap<_, _>>();

    let solutions = client
        .solution_client()
        .find_all(FindAllRequest::default().set_page(0).set_size(100))?;

    let users = solutions
        .iter()
        .filter_map(|s| match problems_by_id.get(&s.problem_id) {
            Some(_) => Some(db::NewAojUser {
                aoj_id: s.user_id.to_string(),
            }),
            None => None,
        })
        .collect::<Vec<_>>();

    info!("Inserting {} users", users.len());
    debug!("{:?}", users);

    db::upsert_aoj_users(&connection, &users);
    let users = db::get_aoj_users_by_aoj_ids(
        &connection,
        &users.into_iter().map(|u| u.aoj_id).collect::<Vec<_>>(),
    );
    let users_by_id = users
        .iter()
        .map(|u| (u.aoj_id.to_string(), u))
        .collect::<HashMap<_, _>>();

    let solutions = solutions
        .into_iter()
        .filter_map(|s| {
            match (
                problems_by_id.get(&s.problem_id),
                users_by_id.get(&s.user_id),
            ) {
                (Some(p), Some(u)) => Some(db::NewAojSolution {
                    aoj_problem_id: p.problem_id,
                    aoj_user_id: u.id,
                }),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    info!("Inserting {} solutions", solutions.len());
    debug!("{:?}", solutions);
    db::upsert_aoj_solutions(&connection, &solutions);

    Ok(())
}