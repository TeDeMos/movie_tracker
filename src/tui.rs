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
            event::{Event, KeyCode, KeyEvent, KeyModifiers},
        },
        layout::{Constraint, Layout, Offset, Spacing},
        style::{Color, Style, Stylize},
        symbols::merge::MergeStrategy,
        widgets::{Block, BorderType, Clear, List, ListState, Paragraph},
    },
    std::{borrow::Cow, fs::File, io::BufRead, ops::Deref},
    textwrap::Options,
};

pub struct App {
    state: AppState,
    watch_list: WatchList,
    add_movie: AddMovie,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum AppState {
    List,
    AddMovie,
    Quit,
}

struct WatchList {
    lines: Vec<String>,
    list_state: ListState,
    expanded: Option<usize>,
}

impl WatchList {
    fn new() -> Result<Self> {
        let lines = File::open_buffered("/home/tedem/dev/typst/movies/movies.txt")?
            .lines()
            .map_while(Result::ok)
            .collect();
        let list_state = ListState::default().with_selected(Some(0));
        let expanded = None;
        Ok(Self { lines, list_state, expanded })
    }

    fn draw(&mut self, app_state: AppState, frame: &mut Frame) {
        let mut items: Vec<_> = self.lines.iter().map(String::deref).collect();
        if let Some(i) = self.expanded {
            items[i] = "Very long\nExpanded\nContent";
        }
        let mut style = Style::default();
        if app_state != AppState::List {
            style.fg = Some(Color::DarkGray);
        }
        let list =
            List::new(items).scroll_padding(3).style(style).highlight_style(style.reversed());
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }

    fn handle_key(&mut self, event: KeyEvent) -> AppState {
        match event.code {
            KeyCode::Char('q') => return AppState::Quit,
            KeyCode::Char('j') => self.list_state.scroll_down_by(1),
            KeyCode::Char('k') => self.list_state.scroll_up_by(1),
            KeyCode::Char('p') => return AppState::AddMovie,
            KeyCode::Enter =>
                if let Some(s) = self.list_state.selected() {
                    self.expanded = self.expanded.is_none_or(|e| e != s).then_some(s);
                },
            _ => {},
        }
        AppState::List
    }
}

struct AddMovie {
    selected: AddMovieTab,
    search: String,
    results: Paginated<SearchMovie>,
    results_state: ListState,
    details_state: ListState,
    half_scroll: usize,
    max_scroll: usize,
}

impl AddMovie {
    fn new() -> Result<Self> {
        let selected = AddMovieTab::Search;
        let search = String::new();
        let results = serde_json::from_reader(File::open_buffered(DEBUG)?)?;
        let results_state = ListState::default().with_selected(Some(0));
        let details_state = ListState::default();
        let half_scroll = 1;
        let max_scroll = 0;
        Ok(Self {
            selected,
            search,
            results,
            results_state,
            details_state,
            half_scroll,
            max_scroll,
        })
    }

    fn change_selection(&mut self, up: bool) {
        let selected = self.results_state.selected_mut().as_mut().unwrap();
        let next = if up {
            selected.saturating_sub(1)
        } else {
            usize::min(self.results.results.len() - 1, *selected + 1)
        };
        if *selected != next {
            *selected = next;
            *self.details_state.offset_mut() = 0;
        }
    }

    fn scroll_preview(&mut self, half: bool, up: bool) {
        let offset = self.details_state.offset_mut();
        let delta = if half { self.half_scroll } else { 1 };
        if up {
            *offset = offset.saturating_sub(delta);
        } else {
            *offset = self.max_scroll.min(*offset + delta);
        }
    }

    fn handle_key(&mut self, event: KeyEvent) -> AppState {
        match self.selected {
            AddMovieTab::Search => match event.code {
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    return AppState::List,
                KeyCode::Esc => return AppState::List,
                KeyCode::Char('k') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.selected = AddMovieTab::Results,
                KeyCode::Enter => self.selected = AddMovieTab::Results,
                KeyCode::Backspace => _ = self.search.pop(),
                KeyCode::Char(c) => self.search.push(c),
                _ => (),
            },
            AddMovieTab::Results => match event.code {
                KeyCode::Char('q') => return AppState::List,
                KeyCode::Char('j') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.selected = AddMovieTab::Search,
                KeyCode::Char('j') => self.change_selection(false),
                KeyCode::Char('k') => self.change_selection(true),
                KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.scroll_preview(true, false),
                KeyCode::Char('u') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.scroll_preview(true, true),
                KeyCode::Char('J') => self.scroll_preview(false, false),
                KeyCode::Char('K') => self.scroll_preview(false, true),
                _ => {},
            },
        }
        AppState::AddMovie
    }

    fn draw(&mut self, app_state: AppState, frame: &mut Frame) {
        if app_state != AppState::AddMovie {
            return;
        }

        let popup_area = Layout::horizontal([1, 7, 1].map(Constraint::Fill))
            .split(Layout::vertical([1, 5, 1].map(Constraint::Fill)).split(frame.area())[1])[1];
        frame.render_widget(Clear, popup_area);

        let [title_area, results_area, search_block_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1), Constraint::Length(3)])
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

        let [list_area, details_block_area] = Layout::horizontal([2, 3].map(Constraint::Fill))
            .spacing(Spacing::Overlap(1))
            .areas(results_area);
        let list_width = list_area.width - 2 - 5;

        let mut list_highlight = Style::default();
        if self.selected != AddMovieTab::Results {
            list_highlight.fg = Some(Color::DarkGray);
        }
        let list = List::new(self.results.results.iter().enumerate().map(|(i, m)| {
            textwrap::fill(
                &match m.release_date {
                    Some(d) => Cow::Owned(format!("{} ({})", m.original_title, d)),
                    None => Cow::Borrowed(&m.original_title),
                },
                Options::new(list_width as usize)
                    .initial_indent(&format!("{:>3}. ", i + 1))
                    .subsequent_indent("     "),
            )
        }))
        .highlight_style(list_highlight.reversed())
        .block(Block::bordered().merge_borders(MergeStrategy::Fuzzy).title(format!(
            "Results (1-{}/{})",
            self.results.results.len(),
            self.results.total_results
        )));

        let details_block = Block::bordered().merge_borders(MergeStrategy::Fuzzy).title("Details");
        let details_inner_area = details_block.inner(details_block_area);

        let text;
        let details = if let Some(s) = self.results_state.selected() {
            text = match self.results.results[s].overview.as_str().trim() {
                "" => Cow::Borrowed("<No overview>"),
                s => Cow::Owned(s.repeat(10)),
            };
            let items = textwrap::wrap(&text, Options::new(details_inner_area.width as usize));
            self.max_scroll = items.len().saturating_sub(details_inner_area.height as _);
            List::new(items)
        } else {
            self.max_scroll = 0;
            List::new::<[&str; 0]>([])
        };
        self.half_scroll = (details_inner_area.height / 2) as _;

        let search_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy)
            .title("Search");

        let search_text_area = search_block.inner(search_block_area);
        let search = Paragraph::new(self.search.as_str());
        if self.selected == AddMovieTab::Search {
            frame.set_cursor_position(
                search_text_area.as_position()
                    + Offset::new(i32::try_from(self.search.len()).unwrap(), 0),
            );
        }

        frame.render_widget(title, title_area);
        frame.render_stateful_widget(list, list_area, &mut self.results_state);
        frame.render_widget(details_block, details_block_area);
        frame.render_stateful_widget(details, details_inner_area, &mut self.details_state);
        frame.render_widget(search_block, search_block_area);
        frame.render_widget(search, search_text_area);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum AddMovieTab {
    Search,
    Results,
}

impl App {
    pub fn new() -> Result<Self> {
        let state = AppState::List;
        let watch_list = WatchList::new()?;
        let add_movie = AddMovie::new()?;
        Ok(Self { state, watch_list, add_movie })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            if self.state == AppState::Quit {
                return Ok(());
            }

            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(k) = event::read()? {
                self.state = match self.state {
                    AppState::List => self.watch_list.handle_key(k),
                    AppState::AddMovie => self.add_movie.handle_key(k),
                    _ => unreachable!(),
                };
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.watch_list.draw(self.state, frame);
        self.add_movie.draw(self.state, frame);
    }
}
