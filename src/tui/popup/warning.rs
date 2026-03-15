use {
    crate::tui::title::Title,
    ratatui::{
        Frame,
        layout::{Constraint, Layout, Rect, Spacing},
        symbols::merge::MergeStrategy,
        widgets::{Block, BorderType, List},
    },
    std::fmt::Display,
};

pub struct Warning(String);

impl Warning {
    pub fn new(e: impl Display) -> Self { Self(e.to_string()) }

    pub fn draw(&mut self, rect: Rect, frame: &mut Frame) {
        let [title, content] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
            .spacing(Spacing::Overlap(1))
            .areas(rect);

        Title("Fatal error").draw(title, frame);
        let block =
            Block::bordered().border_type(BorderType::Rounded).merge_borders(MergeStrategy::Fuzzy);
        frame.render_widget(&block, content);
        let inner = block.inner(content);
        let list = List::new(textwrap::wrap(&self.0, inner.width as usize));
        frame.render_widget(list, inner);
    }
}
