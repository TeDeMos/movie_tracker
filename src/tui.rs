use {
    crate::tmdb::{
        model::{Paginated, SearchMovie},
        utils::DEBUG,
    },
    anyhow::Result,
    ratatui::{
        DefaultTerminal, Frame,
        crossterm::{
            event,
            event::{Event, KeyCode},
        },
        layout::{Constraint, Layout, Spacing},
        style::{Style, Stylize},
        symbols::merge::MergeStrategy,
        widgets::{Block, BorderType, Clear, List, ListState, Paragraph},
    },
    std::{borrow::Cow, fs::File, io::BufRead, ops::Deref},
    textwrap::Options,
};

pub struct App {
    lines: Vec<String>,
    list_state: ListState,
    expanded: Option<usize>,
    movies: Paginated<SearchMovie>,
    popup: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let lines = File::open_buffered("/home/tedem/dev/typst/movies/movies.txt")?
            .lines()
            .map_while(Result::ok)
            .collect();
        let list_state = ListState::default().with_selected(Some(0));
        let expanded = None;
        let movies = serde_json::from_reader(File::open_buffered(DEBUG)?)?;
        let popup = false;
        Ok(Self { lines, list_state, expanded, movies, popup })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') => self.list_state.scroll_down_by(1),
                    KeyCode::Char('k') => self.list_state.scroll_up_by(1),
                    KeyCode::Char('p') => self.popup = !self.popup,
                    KeyCode::Enter if !self.popup =>
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
        if self.popup {
            let popup_area = Layout::horizontal([1, 7, 1].map(Constraint::Fill))
                .split(Layout::vertical([1, 5, 1].map(Constraint::Fill)).split(frame.area())[1])[1];
            frame.render_widget(Clear, popup_area);
            let [title_area, results_area, search_area] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .spacing(Spacing::Overlap(1))
            .areas(popup_area);
            let title = Paragraph::new("Add movie").bold().centered().block(
                Block::bordered()
                    .style(Style::default().not_bold())
                    .border_type(BorderType::Rounded)
                    .merge_borders(MergeStrategy::Exact)
                    .white()
                    .on_black(),
            );
            let [list_area, details_area] = Layout::horizontal([2, 3].map(Constraint::Fill))
                .spacing(Spacing::Overlap(1))
                .areas(results_area);
            let list_width = list_area.width - 2;
            let list = List::new(self.movies.results.iter().map(|m| {
                textwrap::fill(
                    &match m.release_date {
                        Some(d) => Cow::Owned(format!("{} ({})", m.original_title, d)),
                        None => Cow::Borrowed(&m.original_title),
                    },
                    Options::new(list_width as usize).subsequent_indent("  "),
                )
            }))
            .block(Block::bordered().merge_borders(MergeStrategy::Fuzzy).title(format!(
                "Results (1-{}/{})",
                self.movies.results.len(),
                self.movies.total_results
            )));
            let details = Block::bordered().merge_borders(MergeStrategy::Fuzzy).title("Details");
            let search = Block::bordered()
                .border_type(BorderType::Rounded)
                .merge_borders(MergeStrategy::Fuzzy)
                .title("Search");
            frame.render_widget(title, title_area);
            frame.render_widget(list, list_area);
            frame.render_widget(details, details_area);
            frame.render_widget(search, search_area);
        }
    }
}
