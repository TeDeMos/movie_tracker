use {
    crate::tui::{AppAction, Context, utils::KeyResult},
    anyhow::Result,
    ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect},
};

pub struct ConfirmSeries {}

impl ConfirmSeries {
    pub fn new(id: i32, context: Context) -> Self { todo!() }

    pub fn draw(&mut self, rect: Rect, frame: &mut Frame) {}

    pub fn handle_key(&mut self, event: KeyEvent, context: Context) -> KeyResult<AppAction> {
        todo!()
    }

    pub fn handle_client(&mut self, context: Context) -> Result<()> { todo!() }
}
