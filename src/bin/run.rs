extern crate diesel_starter;

fn main() {
    let conn = diesel_starter::establish_connection();
    println!("Hello, world!");
}
