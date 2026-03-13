use ratatui::{
    Frame,
    layout::Rect,
    prelude::Style,
    style::Stylize,
    widgets::{Block, BorderType, Paragraph},
};

pub struct Title<'a>(pub &'a str);

impl Title<'_> {
    pub fn draw(&mut self, rect: Rect, frame: &mut Frame) {
        let title = Paragraph::new(self.0).bold().centered().block(
            Block::bordered().style(Style::default().not_bold()).border_type(BorderType::Rounded),
        );
        frame.render_widget(title, rect);
    }
}
