use aoj_icpc::db;
use serde::Deserialize;

#[derive(Deserialize)]
struct Problem {
    title: String,
    source: String,
    point: i32,
    url: String,
}

fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let conn = db::establish_connection(&database_url);
    let problems: Vec<Problem> =
        serde_json::from_str(include_str!("../../resources/problems.json")).unwrap();
    let problems = problems
        .iter()
        .map(|p| db::NewProblem {
            title: &p.title,
            source: &p.source,
            point: p.point,
            url: &p.url,
        })
        .collect::<Vec<_>>();

    db::replace_problems(&conn, &problems);

    println!("{} initialized", database_url);
}
