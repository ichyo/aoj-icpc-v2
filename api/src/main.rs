use actix_web::{web, App, HttpServer, Responder};
use chrono::prelude::*;
use failure::Error;
use serde::{Deserialize, Serialize};

use aoj_icpc::aoj;
use aoj_icpc::db;
use log::{error, info};
use std::thread;
use std::time;

#[derive(Serialize, Deserialize, Debug)]
struct Problem {
    id: i32,
    point: i32,
    title: String,
    source: String,
    url: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AojUser {
    id: String,
    solutions: Vec<i32>,
}

fn problems(pool: web::Data<db::Pool>) -> impl Responder {
    let connection = pool.get().expect("Failed to get connection from pool");
    let problems = db::get_all_problems(&connection);
    web::Json(
        problems
            .into_iter()
            .map(|p| Problem {
                id: p.id,
                point: p.point,
                title: p.title,
                source: p.source,
                url: p.url,
                status: p.status.to_string(),
            })
            .collect::<Vec<_>>(),
    )
}

fn solutions(pool: web::Data<db::Pool>) -> impl Responder {
    let connection = pool.get().expect("Failed to get connection from pool");
    web::Json(db::get_number_of_solutions_by_problems(&connection))
}

fn aoj_user(pool: web::Data<db::Pool>, aoj_user_id: web::Path<String>) -> impl Responder {
    let connection = pool.get().expect("Failed to get connection from pool");
    let aoj_user_id = aoj_user_id.into_inner();
    let users = db::get_aoj_users_by_aoj_ids(&connection, &[aoj_user_id.clone()]);
    let solutions_by_users = db::get_aoj_solutions_by_aoj_user(&connection, &users);
    match solutions_by_users.get(0) {
        Some((_, solutions)) => web::HttpResponse::Ok().json(AojUser {
            id: aoj_user_id,
            solutions: solutions
                .iter()
                .map(|s| s.aoj_problem_id)
                .collect::<Vec<_>>(),
        }),
        None => web::HttpResponse::NotFound().finish(),
    }
}

fn update_aoj_solutions(pool: db::Pool, duration: time::Duration) -> Result<(), Error> {
    let since = Utc::now() - chrono::Duration::from_std(duration)?;
    let conn = pool.get()?;
    let solutions = aoj::fetch_recent_solutions(10, 100, since)?;
    aoj::insert_solutions(&conn, &solutions)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    env_logger::init_from_env("AOJICPC_LOG");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

    let matches = clap::App::new("AOJ ICPC API Server")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("port number"),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap_or("8080");
    let addr = format!("0.0.0.0:{}", port);

    let pool = db::create_pool(&database_url);

    {
        let pool = pool.clone();
        thread::spawn(move || loop {
            info!("Updating solutions");
            if let Err(e) = update_aoj_solutions(pool.clone(), time::Duration::from_secs(20)) {
                error!("Failed to update solutions: {}", e);
            }
            thread::sleep(time::Duration::from_secs(10));
        });
    }

    info!("Running {}", addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/api/v1/problems").to(problems))
            .service(web::resource("/api/v1/problems/solutions").to(solutions))
            .service(web::resource("/api/v1/aoj_users/{id}").to(aoj_user))
    })
    .bind(addr)?
    .run()
}
