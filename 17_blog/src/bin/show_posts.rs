use chrono::Utc;
use diesel::prelude::*;

use blog::db::get_connection;
use blog::models::post::Post;
use blog::schema::posts::dsl::created_at;
use blog::schema::posts::dsl::posts;

fn main() {
    let conn: &mut _ = &mut get_connection();

    let results = posts
        .filter(created_at.le(Utc::now().naive_utc()))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for row in results {
        let post: Post = row;
        println!("-----------");
        println!("{post:?}");
    }
}
