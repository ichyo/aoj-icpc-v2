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

fn problems(db: web::Data<db::Pool>) -> impl Responder {
    let connection = db.get().expect("Failed to get connection from pool");
    let problems = db::get_problems(&connection);
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

fn aoj_user(_db: web::Data<db::Pool>, _aoj_user_id: web::Path<String>) -> impl Responder {
    web::Json(AojUser {
        solutions: vec![1, 3, 5, 645, 663],
    })
}

fn main() -> std::io::Result<()> {
    env_logger::init_from_env("AOJICPC_LOG");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = db::create_pool(&database_url);

    info!("Running 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/api/v1/problems").to(problems))
            .service(web::resource("/api/v1/aoj_users/{id}").to(aoj_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
