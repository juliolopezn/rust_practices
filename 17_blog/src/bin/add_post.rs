use diesel::associations::HasTable;
use diesel::prelude::*;
use std::io::{stdin, Read};

use blog::db::get_connection;
use blog::models::post::{NewPost, Post};
use blog::schema::posts::dsl::posts;

fn main() {
    let conn: &mut _ = &mut get_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!("\nOk! Let's write {title} (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut body).unwrap();

    let new_post = NewPost {
        title,
        body: &body,
        slug: &slug::slugify(title),
    };

    let post = diesel::insert_into(posts::table())
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post");

    println!("\nSaved draft {title} with id {post:?}");
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
