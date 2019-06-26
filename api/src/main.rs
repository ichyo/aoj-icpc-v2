use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use aoj_icpc::db;

#[derive(Serialize, Deserialize, Debug)]
struct Problem {
    point: i32,
    title: String,
    source: String,
    solutions: u32,
    url: String,
    stars: u16,
}

fn problems(db: web::Data<db::Pool>) -> impl Responder {
    let connection = db.get().expect("Failed to get connection from pool");
    let problems = db::get_problems(&connection);
    web::Json(
        problems
            .into_iter()
            .map(|p| Problem {
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

fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = db::create_pool(&database_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/api/v1/problems").to(problems))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
