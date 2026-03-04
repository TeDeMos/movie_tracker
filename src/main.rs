#![feature(file_buffered)]
#![warn(clippy::pedantic)]

use crate::tmdb::client::Client;

mod tmdb;

fn main() {
    let client = Client::new();
    // let movie = client.search_movie("everything everywhere", 1).unwrap();
    let movie = client.movie(545_611).unwrap();
    dbg!(movie);
}
