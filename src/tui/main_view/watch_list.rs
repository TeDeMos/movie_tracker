use {
    crate::tui::{AppAction, KeyResult, popup::Popup},
    anyhow::Result,
    ratatui::{
        Frame,
        crossterm::event::{KeyCode, KeyEvent},
        prelude::{Color, Style},
        widgets::{List, ListState},
    },
    std::{fs::File, io::BufRead},
};

pub struct WatchList {
    lines: Vec<String>,
    list_state: ListState,
    expanded: Option<usize>,
}

impl WatchList {
    pub fn new() -> Result<Self> {
        let lines = File::open_buffered("/home/tedem/dev/typst/movies/movies.txt")?
            .lines()
            .map_while(Result::ok)
            .collect();
        let list_state = ListState::default().with_selected(Some(0));
        let expanded = None;
        Ok(Self { lines, list_state, expanded })
    }

    pub fn draw(&mut self, covered: bool, frame: &mut Frame) {
        let mut items: Vec<_> = self.lines.iter().map(String::as_str).collect();
        if let Some(i) = self.expanded {
            items[i] = "Very long\nExpanded\nContent";
        }
        let style = if covered { Style::default().fg(Color::DarkGray) } else { Style::default() };
        let list =
            List::new(items).scroll_padding(3).style(style).highlight_style(style.reversed());
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }

    pub fn handle_key(&mut self, event: KeyEvent) -> KeyResult<AppAction> {
        match event.code {
            KeyCode::Char('j') => {
                self.list_state.scroll_down_by(1);
                KeyResult::Consumed
            },
            KeyCode::Char('k') => {
                self.list_state.scroll_up_by(1);
                KeyResult::Consumed
            },
            KeyCode::Enter => {
                let s = self.list_state.selected().unwrap();
                self.expanded = self.expanded.is_none_or(|e| e != s).then_some(s);
                KeyResult::Consumed
            },
            KeyCode::Char('m') => KeyResult::Action(AppAction::ShowPopup(Popup::add_movie())),
            _ => KeyResult::Propagate(event),
        }
    }
}
