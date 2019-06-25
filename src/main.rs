use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Problem {
    point: u16,
    title: String,
    source: String,
    solutions: u32,
    url: String,
    stars: u16,
}

fn problems() -> impl Responder {
    let problems = vec![Problem {
        point: 100,
        title: "ICPC 得点集計ソフトウェア".to_string(),
        source: "国内予選2007A".to_string(),
        solutions: 2692,
        url: "http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1147&lang=jp".to_string(),
        stars: 0,
    }];
    web::Json(problems)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/api/v1/problems").to(problems)))
        .bind("0.0.0.0:8080")?
        .run()
}