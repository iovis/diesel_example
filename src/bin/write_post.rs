use std::io::{stdin, Read};

use diesel_example::{create_post, establish_connection};

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("Title:");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nBody: (Press {EOF} when finished)\n");
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\nSaved draft {} with id {}", post.title, post.id);
}
