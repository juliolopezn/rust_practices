use std::env;

use actix_web::{error, web::{self, Data}, App, HttpResponse, HttpServer};
use blog::db;
use dotenvy::dotenv;

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
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(
                    err,
                    HttpResponse::Conflict().finish(),
                )
                .into()
            });
        let pool: db::PoolConnection = db::get_pool_connection();

        App::new()
            .app_data(json_config)
            .app_data(Data::new(pool))
            .service(web::scope("/posts").configure(blog::routes::posts::config))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
