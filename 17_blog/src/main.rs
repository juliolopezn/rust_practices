use std::env;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use blog::PoolConnection;
use dotenvy::dotenv;
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16 integer");

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    HttpServer::new(|| {
        let json_config = actix_web::web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                actix_web::error::InternalError::from_response(err, actix_web::HttpResponse::Conflict().finish())
                    .into()
            });
        let pool: PoolConnection = blog::get_pool_connection();

        App::new()
            .app_data(json_config)
            .app_data(Data::new(pool))
            .service(health)
            .service(posts_get)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[derive(Serialize)]
enum HealthCheckStatus {
    UP,
    DOWN,
}

#[derive(Serialize)]
struct HealthCheck {
    timestamp: String,
    database: HealthCheckStatus,
}

#[get("/health")]
async fn health(pool: Data<PoolConnection>) -> impl Responder {
    let health_check = HealthCheck {
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: match pool.get() {
            Ok(_) => HealthCheckStatus::UP,
            Err(_) => HealthCheckStatus::DOWN,
        },
    };
    HttpResponse::Ok().json(health_check)
}


#[get("/posts")]
async fn posts_get(pool: Data<PoolConnection>) -> impl Responder {
    use blog::models::Post;
    use blog::schema::posts::dsl::posts;
    use diesel::prelude::*;

    let conn: &mut _ = &mut pool.get().expect("Couldn't get db connection from pool");

    let results = posts
        // .limit(limit as i64)
        // .offset(offset as i64)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(results)
}
