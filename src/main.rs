#![feature(file_buffered)]
#![warn(clippy::pedantic)]

use {
    crate::{db::Database, tmdb::client::TmdbClient, tui::App},
    anyhow::Result,
    std::{thread, time::Duration},
};

mod db;
mod tmdb;
mod tui;

fn main() -> Result<()> {
    // let mut db = Database::new()?;
    // let mut client = TmdbClient::new();
    // let id = client.search_movie("Fargo".into(), 1);
    // let search = loop {
    //     match client.search_movie_results(id) {
    //         None => thread::sleep(Duration::from_millis(20)),
    //         Some(r) => break r?,
    //     }
    // };
    // let id = client.movie(search.results.first().unwrap().id);
    // let movie = loop {
    //     match client.movie_results(id) {
    //         None => thread::sleep(Duration::from_millis(20)),
    //         Some(r) => break r?,
    //     }
    // };
    // db.insert_movie(&movie)?;
    // Ok(())
    let terminal = ratatui::init();
    let result = App::new()?.run(terminal);
    ratatui::restore();
    result
}
