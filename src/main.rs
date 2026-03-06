#![feature(file_buffered)]
#![warn(clippy::pedantic)]

use {
    crate::{tmdb::client::Client, tui::App},
    anyhow::Result,
};

mod tmdb;
mod tui;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new()?.run(terminal);
    ratatui::restore();
    result
    // let client = Client::new();
    // let movie = client.search_movie("everything everywhere", 1).unwrap();
    // let movie = client.movie(545_611).unwrap();
    // dbg!(movie);
    // let series = client.search_tv("breaking", 1).unwrap();
    // let series = client.tv_series(1396).unwrap();
    // dbg!(series);
    // let season = client.tv_season(1396, 1).unwrap();
    // dbg!(season);
    // let episode = client.tv_episode(1396, 1, 1).unwrap();
    // dbg!(episode);
    // Ok(())
}
