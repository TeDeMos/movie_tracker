use {
    crate::tui::utils::{ScrollDirection, ScrollOffset},
    ratatui::{
        Frame,
        layout::Rect,
        symbols::merge::MergeStrategy,
        widgets::{Block, List, ListState},
    },
};

pub struct Details {
    half_view: usize,
    max_offset: usize,
    list_state: ListState,
}

impl Details {
    pub fn new() -> Self { Self { half_view: 0, max_offset: 0, list_state: ListState::default() } }

    pub fn draw(&mut self, overview: Option<&str>, rect: Rect, frame: &mut Frame) {
        let block = Block::bordered().merge_borders(MergeStrategy::Fuzzy).title("Details");
        frame.render_widget(&block, rect);

        let inner = block.inner(rect);
        self.half_view = (inner.height / 2) as _;
        let offset = self.list_state.offset_mut();
        if let Some(s) = overview {
            let lines = textwrap::wrap(s, inner.width as usize);
            self.max_offset = lines.len().saturating_sub(inner.height as _);
            *offset = self.max_offset.min(*offset);
            let list = List::new(lines);
            frame.render_stateful_widget(list, inner, &mut self.list_state);
        } else {
            self.max_offset = 0;
            *offset = 0;
        }
    }

    pub fn reset_offset(&mut self) { *self.list_state.offset_mut() = 0; }

    pub fn change_offset(&mut self, direction: ScrollDirection, offset: ScrollOffset) {
        let delta = match offset {
            ScrollOffset::HalfView => self.half_view,
            ScrollOffset::One => 1,
        };
        let offset = self.list_state.offset_mut();
        match direction {
            ScrollDirection::Up => *offset = offset.saturating_sub(delta),
            ScrollDirection::Down => *offset = self.max_offset.min(*offset + delta),
        }
    }
}
