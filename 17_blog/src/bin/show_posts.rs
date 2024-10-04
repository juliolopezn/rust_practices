use blog::get_pool_connection;
use blog::models::Post;
use blog::schema::posts::dsl::posts;
use chrono::Utc;
use diesel::prelude::*;

fn main() {
    use blog::schema::posts::dsl::created_at;

    let pool = get_pool_connection();
    let conn= &mut pool.get().expect("Couldn't get db connection from pool");

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
