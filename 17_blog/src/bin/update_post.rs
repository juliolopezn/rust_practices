use diesel::prelude::*;
use std::io::{stdin, Read};
use uuid::Uuid;

use blog::db::get_connection;
use blog::models::post::{NewPost, Post};
use blog::schema::posts::dsl;
use blog::schema::posts::dsl::posts;

fn main() {
    let conn: &mut _ = &mut get_connection();

    let mut page: u32 = 1;
    let page_size: u32 = 5;

    let mut post_ids: Vec<Uuid> = vec![];

    println!("Choose post to update [ENTER to load more, {EOF} to exit]");
    loop {
        let offset = (page - 1) * page_size;
        let limit = page * page_size;

        let results = posts
            .limit(limit as i64)
            .offset(offset as i64)
            .select(Post::as_select())
            .load(conn)
            .expect("Error loading posts");

        match results.is_empty() {
            true => println!("No more posts to show"),
            false => {
                for (index, post) in results.iter().enumerate() {
                    println!("({}) - {}", index + offset as usize + 1, post.title);
                    post_ids.push(post.id);
                }
            }
        }

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "" => page += 1,
            EOF => break,
            _ => {
                let index = choice.trim().parse::<usize>().unwrap();
                let post_id = &post_ids[index];

                println!("Updating post with ID ({})", post_id);

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

                let post = diesel::update(posts.find(post_id))
                    .filter(dsl::id.eq(post_id))
                    .set((dsl::title.eq(new_post.title), dsl::body.eq(new_post.body)))
                    .returning(Post::as_returning())
                    .get_result(conn)
                    .expect("Error updating new post");

                println!("\nUpdated {title} with id {post:?}");

                break;
            }
        }
    }
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
