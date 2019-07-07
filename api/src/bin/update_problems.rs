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
    year: i16,
    source_type: String,
}

fn to_problem_status(status: &str) -> db::ProblemStatus {
    match status {
        "active" => db::ProblemStatus::Active,
        "hidden" => db::ProblemStatus::Hidden,
        "pending" => db::ProblemStatus::Pending,
        _ => unreachable!(),
    }
}

fn to_source_type(s: &str) -> db::SourceType {
    match s {
        "domestic" => db::SourceType::Domestic,
        "regional" => db::SourceType::Regional,
        "jag_domestic" => db::SourceType::JagDomestic,
        "jag_regional" => db::SourceType::JagRegional,
        "others" => db::SourceType::Others,
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
    dotenv::dotenv().ok();
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
            source_type: to_source_type(&p.source_type),
            year: p.year,
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
        "Updating {} problems ({} aojs)",
        problems.len(),
        aoj_problems.len()
    );

    db::update_problems(&conn, &problems, &aoj_problems);

    info!("update problems");
}
