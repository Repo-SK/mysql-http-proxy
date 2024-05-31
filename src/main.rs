mod middleware;

use std::env;

use dotenvy::dotenv;
use ntex::web;
use serde::Serialize;
use sqlx::{Column, MySql, MySqlPool, Pool, Row, TypeInfo};
use crate::middleware::auth::AuthMiddleware;

struct AppState {
    db_pool: Pool<MySql>,
}

#[derive(Serialize)]
struct QueryResponse {
    result: serde_json::Value,
}

#[web::post("/query")]
async fn query(query: web::types::Json<String>, state: web::types::State<AppState>) -> impl web::Responder {
    let result = sqlx::query(&query)
        .fetch_all(&state.db_pool)
        .await;

    match result {
        Ok(rows) => {
            let json_result: Vec<serde_json::Value> = rows.iter()
                .map(|row| {
                    let mut map = serde_json::Map::new();
                    for column in row.columns() {
                        let column_name = column.name();
                        let type_name = column.type_info().name();
                        let value = match type_name {
                            "TEXT" | "VARCHAR" | "CHAR" => row.try_get::<String, &str>(column_name).unwrap_or_default().into(),
                            "INT" | "INTEGER" => row.try_get::<i32, &str>(column_name).unwrap_or(0).into(),
                            "BIGINT" => row.try_get::<i64, &str>(column_name).unwrap_or(0).into(),
                            "BIGINT UNSIGNED" => row.try_get::<u64, &str>(column_name).unwrap_or(0).into(),
                            "FLOAT" => row.try_get::<f32, &str>(column_name).unwrap_or(0.0).into(),
                            "DOUBLE" => row.try_get::<f64, &str>(column_name).unwrap_or(0.0).into(),
                            "BOOLEAN" => row.try_get::<bool, &str>(column_name).unwrap_or(false).into(),
                            _ => serde_json::Value::Null,
                        };
                        map.insert(column_name.to_string(), value);
                    }
                    serde_json::Value::Object(map)
                })
                .collect();

            web::HttpResponse::Ok().json(&QueryResponse {
                result: serde_json::Value::Array(json_result),
            })
        },
        Err(e) => web::HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}


#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>().expect("PORT must be a valid number");

    let bearer_token = env::var("BEARER_TOKEN")
        .expect("BEARER_TOKEN must be set");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db_pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    web::HttpServer::new(move || {
        web::App::new()
            .state(AppState {
                db_pool: db_pool.clone()
            })
            .wrap(AuthMiddleware::new(bearer_token.clone()))
            .service(query)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}