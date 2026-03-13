use {
    crate::{
        tmdb::client::TmdbClient,
        tui::{
            AppAction,
            popup::add_movie::AddMovie,
            utils::{EventExt, KeyResult},
        },
    },
    ratatui::{
        Frame,
        crossterm::event::{KeyCode, KeyEvent},
        layout::{Constraint, Layout},
        widgets::Clear,
    },
};

mod add_movie;

pub enum Popup {
    AddMovie(AddMovie),
}

impl Popup {
    pub fn add_movie() -> Self { Self::AddMovie(AddMovie::new()) }

    pub fn draw(&mut self, frame: &mut Frame) {
        let rect = Layout::horizontal([1, 7, 1].map(Constraint::Fill))
            .split(Layout::vertical([1, 5, 1].map(Constraint::Fill)).split(frame.area())[1])[1];
        frame.render_widget(Clear, rect);

        match self {
            Popup::AddMovie(a) => a.draw(rect, frame),
        }
    }

    pub fn handle_key(&mut self, event: KeyEvent, client: &mut TmdbClient) -> KeyResult<AppAction> {
        let result = match self {
            Popup::AddMovie(a) => a.handle_key(event, client),
        };
        result.handle_propagate(|event| match event.code {
            KeyCode::Esc | KeyCode::Char('q') if event.no_modifiers() =>
                KeyResult::Action(AppAction::HidePopup),
            _ => KeyResult::Propagate(event),
        })
    }

    pub fn handle_client(&mut self, client: &mut TmdbClient) {
        match self {
            Popup::AddMovie(a) => a.handle_client(client),
        }
    }
}
