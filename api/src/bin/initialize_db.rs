use aoj_icpc::db;
use log::info;
use serde::Deserialize;

use std::str::FromStr;
use url::Url;

#[derive(Deserialize)]
struct Problem {
    id: i32,
    title: String,
    source: String,
    point: i32,
    url: String,
    status: String,
}

fn to_problem_status(status: &str) -> db::ProblemStatus {
    match status {
        "active" => db::ProblemStatus::Active,
        "hidden" => db::ProblemStatus::Hidden,
        "pending" => db::ProblemStatus::Pending,
        _ => unreachable!(),
    }
}

fn to_aoj_id(problem_url: &str) -> Option<String> {
    if let Ok(url) = Url::from_str(problem_url) {
        let domain = url.domain();
        let id_query = url.query_pairs().find(|(key, _)| key == "id");
        if let (Some("judge.u-aizu.ac.jp"), Some(id_query)) = (domain, id_query) {
            return Some(id_query.1.to_string());
        }
    }
    None
}

fn main() {
    env_logger::init_from_env("AOJICPC_LOG");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let conn = db::establish_connection(&database_url);
    let problems: Vec<Problem> =
        serde_json::from_str(include_str!("../../resources/problems.json")).unwrap();

    let problems = problems
        .into_iter()
        .map(|p| db::Problem {
            id: p.id,
            title: p.title,
            source: p.source,
            point: p.point,
            url: p.url,
            status: to_problem_status(&p.status),
        })
        .collect::<Vec<_>>();

    let aoj_problems = problems
        .iter()
        .filter_map(|p| {
            to_aoj_id(&p.url).map(|aoj_id| db::AojProblem {
                problem_id: p.id,
                aoj_id,
            })
        })
        .collect::<Vec<_>>();

    info!(
        "Adding {} problems ({} aojs)",
        problems.len(),
        aoj_problems.len()
    );

    db::initialize_problems(&conn, &problems, &aoj_problems);

    info!("{} initialized", database_url);
}
