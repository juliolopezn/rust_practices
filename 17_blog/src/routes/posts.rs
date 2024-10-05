use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

use crate::models::post::Post;
use crate::db::PoolConnection;
use crate::schema::posts::dsl::posts;

#[derive(Deserialize)]
struct Page {
    limit: u32,
    offset: u32,
}

impl Default for Page {
    fn default() -> Self {
        Self { limit: 10, offset: 0 }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(posts_get);
}

#[get("")]
async fn posts_get(pool: web::Data<PoolConnection>, query: web::Query<Page>) -> impl Responder {
    let conn: &mut _ = &mut pool.get().expect("Couldn't get db connection from pool");

    let results = posts
        .limit(query.limit as i64)
        .offset(query.offset as i64)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(results)
}
