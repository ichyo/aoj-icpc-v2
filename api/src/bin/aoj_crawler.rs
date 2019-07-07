use aoj_icpc::db;
use aoj_icpc::aoj;
use chrono::prelude::*;
use failure::Error;

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init_from_env("AOJICPC_LOG");

    let matches = clap::App::new("AOJ Solution Crawler")
        .version("0.1.0")
        .args(&[
            clap::Arg::with_name("all")
                .long("all")
                .takes_value(false)
                .help("all problems"),
            clap::Arg::with_name("size")
                .long("size")
                .takes_value(true)
                .help("size to fetch"),
            clap::Arg::with_name("seconds")
                .long("seconds")
                .takes_value(true)
                .help("since N seconds ago"),
        ])
        .get_matches();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let connection = db::establish_connection(&database_url);

    let since = Utc::now()
        - chrono::Duration::seconds(
            matches
                .value_of("seconds")
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0),
        );

    let solutions = if matches.occurrences_of("all") > 0 {
        aoj::fetch_all_solutions(&connection)?
    } else {
        aoj::fetch_recent_solutions(
            matches
                .value_of("size")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(100),
            100,
            since,
        )?
    };
    aoj::insert_solutions(&connection, &solutions)
}
