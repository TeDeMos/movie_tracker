use {
    crate::tui::{KeyResult, utils::EventExt},
    ratatui::{
        Frame,
        crossterm::event::{KeyCode, KeyEvent},
        layout::{Offset, Rect},
        symbols::merge::MergeStrategy,
        widgets::{Block, BorderType, Paragraph},
    },
};

pub struct Search {
    query: String,
    chars: usize,
}

impl Search {
    pub fn new() -> Self { Self { query: String::new(), chars: 0 } }

    pub fn query(&self) -> &str { &self.query }

    pub fn draw(&mut self, active: bool, rect: Rect, frame: &mut Frame) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy)
            .title("Search");
        frame.render_widget(&block, rect);

        let inner = block.inner(rect);
        let search = Paragraph::new(self.query.as_str());
        frame.render_widget(search, inner);

        if active {
            frame.set_cursor_position(inner.as_position() + self.cursor_offset());
        }
    }

    pub fn handle_key(&mut self, event: KeyEvent) -> KeyResult<Searched> {
        match event.code {
            KeyCode::Enter if event.no_modifiers() => KeyResult::Action(Searched),
            KeyCode::Backspace if event.no_modifiers() => {
                _ = self.query.pop();
                self.chars = self.chars.saturating_sub(1);
                KeyResult::Consumed
            },
            KeyCode::Char(c) if event.shift_or_no_modifiers() => {
                self.query.push(c);
                self.chars += 1;
                KeyResult::Consumed
            },
            _ => KeyResult::Propagate(event),
        }
    }

    fn cursor_offset(&self) -> Offset { Offset::new(self.chars as i32, 0) }
}

pub struct Searched;
