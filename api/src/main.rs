use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use aoj_icpc::db;
use log::info;

#[derive(Serialize, Deserialize, Debug)]
struct Problem {
    id: i32,
    point: i32,
    title: String,
    source: String,
    solutions: u32,
    url: String,
    stars: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct AojUser {
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
                solutions: 0,
                url: p.url,
                stars: 0,
            })
            .collect::<Vec<_>>(),
    )
}

fn aoj_user(pool: web::Data<db::Pool>, aoj_user_id: web::Path<String>) -> impl Responder {
    let connection = pool.get().expect("Failed to get connection from pool");
    let aoj_user_id = aoj_user_id.into_inner();
    let users = db::get_aoj_users_by_aoj_ids(&connection, &[aoj_user_id]);
    let solutions_by_users = db::get_aoj_solutions_by_aoj_user(&connection, &users);
    match solutions_by_users.get(0) {
        Some((_, solutions)) => web::HttpResponse::Ok().json(AojUser {
            solutions: solutions
                .iter()
                .map(|s| s.aoj_problem_id)
                .collect::<Vec<_>>(),
        }),
        None => web::HttpResponse::NotFound().finish(),
    }
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

    let port = matches.value_of("port").unwrap_or("8000");
    let addr = format!("0.0.0.0:{}", port);

    let pool = db::create_pool(&database_url);

    info!("Running {}", addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/api/v1/problems").to(problems))
            .service(web::resource("/api/v1/aoj_users/{id}").to(aoj_user))
    })
    .bind(addr)?
    .run()
}
