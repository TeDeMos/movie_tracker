use {
    crate::{
        tmdb::{
            client::TmdbClient,
            model::{SearchMovie, SearchTv},
        },
        tui::{
            AppAction, Context,
            popup::{
                confirm_movie::ConfirmMovie, confirm_series::ConfirmSeries,
                paginated_search::PaginatedSearch, warning::Warning,
            },
            utils::{EventExt, IntoAction, KeyResult},
        },
    },
    anyhow::Result,
    derive_more::From,
    ratatui::{
        Frame,
        crossterm::event::{KeyCode, KeyEvent},
        layout::{Constraint, Layout},
        widgets::Clear,
    },
    std::{error::Error, fmt::Display},
};

mod confirm_movie;
mod confirm_series;
mod paginated_search;
mod warning;

#[derive(From)]
pub enum Popup {
    SearchMovie(PaginatedSearch<SearchMovie>),
    SearchTv(PaginatedSearch<SearchTv>),
    ConfirmMovie(ConfirmMovie),
    ConfirmSeries(ConfirmSeries),
    Warning(Warning),
}

impl Popup {
    pub fn search_movie() -> Self { Self::SearchMovie(PaginatedSearch::new()) }

    pub fn search_tv() -> Self { Self::SearchTv(PaginatedSearch::new()) }

    pub fn warning(e: impl Display) -> Self { Warning::new(e).into() }

    pub fn confirm_movie(id: i32, context: Context) -> Self {
        match ConfirmMovie::new(id, context) {
            Ok(c) => c.into(),
            Err(e) => Self::warning(e),
        }
    }

    pub fn confirm_series(id: i32, context: Context) -> Self {
        ConfirmSeries::new(id, context).into()
    }

    pub fn show(self) -> AppAction { AppAction::ShowPopup(self) }

    pub fn draw(&mut self, frame: &mut Frame) {
        let rect = Layout::horizontal([1, 7, 1].map(Constraint::Fill))
            .split(Layout::vertical([1, 5, 1].map(Constraint::Fill)).split(frame.area())[1])[1];
        frame.render_widget(Clear, rect);

        match self {
            Self::SearchMovie(s) => s.draw(rect, frame),
            Self::SearchTv(s) => s.draw(rect, frame),
            Self::ConfirmMovie(d) => d.draw(rect, frame),
            Self::ConfirmSeries(d) => d.draw(rect, frame),
            Self::Warning(e) => e.draw(rect, frame),
        }
    }

    pub fn handle_key(&mut self, event: KeyEvent, context: Context) -> KeyResult<AppAction> {
        let result = match self {
            Self::SearchMovie(s) => s.handle_key(event, context),
            Self::SearchTv(s) => s.handle_key(event, context),
            Self::ConfirmMovie(d) => d.handle_key(event, context),
            Self::ConfirmSeries(d) => d.handle_key(event, context),
            Self::Warning(_) => event.into(),
        };
        result.or_handle_key(|event| match event.code {
            KeyCode::Esc | KeyCode::Char('q') if event.no_modifiers() =>
                AppAction::HidePopup.action(),
            _ => event.into(),
        })
    }

    pub fn handle_client(&mut self, context: Context) -> Result<()> {
        match self {
            Self::SearchMovie(s) => s.handle_client(context),
            Self::SearchTv(s) => s.handle_client(context),
            Self::ConfirmMovie(d) => d.handle_client(context),
            Self::ConfirmSeries(d) => d.handle_client(context),
            Self::Warning(_) => Ok(()),
        }
    }
}
