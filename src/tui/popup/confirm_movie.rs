use {
    crate::{
        db::model::MovieDetails,
        tui::{AppAction, Context, utils::KeyResult},
    },
    anyhow::Result,
    ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect},
};

#[expect(clippy::large_enum_variant)]
pub enum ConfirmMovie {
    Loading(usize),
    Loaded(MovieDetails),
}

impl ConfirmMovie {
    pub fn new(id: i32, context: Context) -> Result<Self> {
        match context.database.get_movie_details(id)? {
            Some(m) => Ok(Self::Loaded(m)),
            None => Ok(Self::Loading(context.client.movie(id))),
        }
    }

    pub fn draw(&mut self, rect: Rect, frame: &mut Frame) {}

    pub fn handle_key(&mut self, event: KeyEvent, context: Context) -> KeyResult<AppAction> {
        event.into()
    }

    pub fn handle_client(&mut self, context: Context) -> Result<()> {
        let Self::Loading(id) = self else { return Ok(()) };
        let movie = match context.client.movie_results(*id) {
            Some(r) => r?,
            None => return Ok(()),
        };
        context.database.insert_movie(&movie)?;
        let details = context.database.get_movie_details(movie.id)?.unwrap();
        *self = Self::Loaded(details);
        Ok(())
    }
}
