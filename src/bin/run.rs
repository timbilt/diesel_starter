extern crate diesel_starter;

use diesel::prelude::*;
use diesel_starter::{connect, models::*, schema};

fn main() {
    use schema::posts::dsl::*;

    let connection: &mut PgConnection = &mut connect::establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
