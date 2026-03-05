use {
    anyhow::Result,
    ratatui::{
        DefaultTerminal, Frame,
        crossterm::{
            event,
            event::{Event, KeyCode},
        },
        style::Style,
        widgets::{List, ListState},
    },
    std::{fs::File, io::BufRead, ops::Deref},
};

pub struct App {
    lines: Vec<String>,
    list_state: ListState,
    expanded: Option<usize>,
}

impl App {
    pub fn new() -> Result<Self> {
        let lines = File::open_buffered("/home/tedem/dev/typst/movies/movies.txt")?
            .lines()
            .map_while(Result::ok)
            .collect();
        let list_state = ListState::default().with_selected(Some(0));
        let expanded = None;
        Ok(Self { lines, list_state, expanded })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') => self.list_state.scroll_down_by(1),
                    KeyCode::Char('k') => self.list_state.scroll_up_by(1),
                    KeyCode::Enter =>
                        if let Some(s) = self.list_state.selected() {
                            self.expanded = self.expanded.is_none_or(|e| e != s).then_some(s);
                        },
                    _ => {},
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut items: Vec<_> = self.lines.iter().map(String::deref).collect();
        if let Some(i) = self.expanded {
            items[i] = "Very long\nExpanded\nContent";
        }
        let list = List::new(items).scroll_padding(3).highlight_style(Style::new().reversed());
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }
}
