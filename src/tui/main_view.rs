use {
    crate::tui::{AppAction, main_view::watch_list::WatchList, utils::KeyResult},
    anyhow::Result,
    ratatui::{Frame, crossterm::event::KeyEvent},
};

mod watch_list;

pub enum MainView {
    WatchList(WatchList),
}

impl MainView {
    pub fn new() -> Result<Self> { WatchList::new().map(Self::WatchList) }

    pub fn draw(&mut self, covered: bool, frame: &mut Frame) {
        match self {
            MainView::WatchList(w) => w.draw(covered, frame),
        }
    }

    pub fn handle_key(&mut self, event: KeyEvent) -> KeyResult<AppAction> {
        match self {
            MainView::WatchList(w) => w.handle_key(event),
        }
    }
}
