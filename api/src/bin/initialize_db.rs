use aoj_icpc::db;

fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let conn = db::establish_connection(&database_url);
    let problems = vec![db::NewProblem {
        title: "abc",
        source: "aaa",
        point: 120,
        url: "https://doc.rust-lang.org/cargo/reference/manifest.html#the-project-layout",
    }];
    db::replace_problems(&conn, &problems);
}