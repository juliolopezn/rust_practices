use blog::establish_connection;
use blog::models::Post;
use blog::schema::posts::dsl::posts;
use chrono::Utc;
use diesel::prelude::*;

fn main() {
    use blog::schema::posts::dsl::created_at;

    let connection = &mut establish_connection();
    let results = posts
        .filter(created_at.le(Utc::now().naive_utc()))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for row in results {
        let post: Post = row;
        println!("-----------");
        println!("{post:?}");
    }
}
