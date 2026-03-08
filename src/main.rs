#![feature(file_buffered)]
#![warn(clippy::pedantic)]

use {crate::tui::App, anyhow::Result};

mod tmdb;
mod tui;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new()?.run(terminal);
    ratatui::restore();
    result
}
