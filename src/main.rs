#![feature(file_buffered)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use {crate::tui::App, anyhow::Result};

mod db;
mod tmdb;
mod tui;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new()?.run(terminal);
    ratatui::restore();
    result
}
