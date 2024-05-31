mod middleware;

use std::env;

use dotenvy::dotenv;
use ntex::web;
use crate::middleware::auth::AuthMiddleware;

#[web::post("/query")]
async fn query(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>().expect("PORT must be a valid number");

    let bearer_token = env::var("BEARER_TOKEN")
        .expect("BEARER_TOKEN must be set");

    web::HttpServer::new(move || {
        web::App::new()
            .wrap(AuthMiddleware::new(bearer_token.clone()))
            .service(query)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}