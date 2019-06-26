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
    let problems = vec![
        Problem {
            point: 100,
            title: "Hanafuda Shuffle".to_string(),
            source: "国内予選2004A".to_string(),
            solutions: 2076,
            url: "http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1129&lang=jp"
                .to_string(),
            stars: 0,
        },
        Problem {
            point: 150,
            title: "Red and Black".to_string(),
            source: "国内予選2004B".to_string(),
            solutions: 1887,
            url: "http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1130&lang=jp"
                .to_string(),
            stars: 0,
        },
    ];

    web::Json(problems)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/api/v1/problems").to(problems)))
        .bind("0.0.0.0:8080")?
        .run()
}
