use actix_web::{error, get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
            
        App::new()
            .app_data(json_config)
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[derive(Serialize)]
struct HealthCheck {
    timestamp: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    let health_check = HealthCheck {
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    HttpResponse::Ok().json(health_check)
}
