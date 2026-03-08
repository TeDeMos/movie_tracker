use {
    crate::tmdb::{client::TmdbClient, model::SearchMovie},
    anyhow::Result,
    ratatui::{
        DefaultTerminal, Frame,
        crossterm::{
            event,
            event::{Event, KeyCode, KeyEvent, KeyModifiers},
        },
        layout::{Constraint, Layout, Offset, Rect, Spacing},
        style::{Color, Style, Stylize},
        symbols::merge::MergeStrategy,
        text::{Line, Text},
        widgets::{Block, BorderType, Clear, List, ListState, Paragraph},
    },
    std::{borrow::Cow, fs::File, io::BufRead, ops::Deref, time::Duration},
    textwrap::Options,
};

pub struct App {
    state: AppState,
    watch_list: WatchList,
    add_movie: AddMovie,
    client: TmdbClient,
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
    current_window: AddMovieWindow,
    search: String,
    results: Results,
    details: Details,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum AddMovieWindow {
    Search,
    Results,
}

struct Results {
    movies: Vec<SearchMovie>,
    next_page: i32,
    total: usize,
    query: String,
    list_state: ListState,
    loading: Option<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ScrollDirection {
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ScrollOffset {
    One,
    HalfView,
}

impl Results {
    fn new() -> Self {
        Self {
            movies: Vec::new(),
            next_page: 0,
            total: 0,
            query: String::new(),
            list_state: ListState::default(),
            loading: None,
        }
    }

    fn change_selected(&mut self, direction: ScrollDirection) -> bool {
        let selected = self.list_state.selected_mut().get_or_insert(0);
        let next = match direction {
            ScrollDirection::Up => selected.saturating_sub(1),
            ScrollDirection::Down => {
                let items = self.movies.len();
                let max_selected =
                    items.saturating_sub((self.loading.is_none() && (self.total <= items)).into());
                max_selected.min(*selected + 1)
            },
        };
        let changed = *selected != next;
        *selected = next;
        changed
    }

    fn list_items(&self, width: usize) -> impl Iterator<Item = Text<'static>> {
        const INDENT: usize = 5;
        const SPACES: &str = unsafe { str::from_utf8_unchecked(&[b' '; INDENT]) };
        const FILL: usize = INDENT - 2;
        self.movies
            .iter()
            .enumerate()
            .map(move |(i, m)| {
                Text::from(textwrap::fill(
                    &match m.release_date {
                        Some(d) => format!("{} ({d})", m.original_title),
                        None => format!("{}, (?)", m.original_title),
                    },
                    Options::new(width - FILL - 2)
                        .initial_indent(&format!("{:>FILL$}. ", i + 1))
                        .subsequent_indent(SPACES),
                ))
            })
            .chain(
                self.loading
                    .map(|_| "Loading...")
                    .or_else(|| (self.total > self.movies.len()).then_some("[Load more]"))
                    .map(|s| Text::from(s).italic().bold().centered()),
            )
    }

    fn list_title(&self) -> Line<'static> {
        if self.movies.is_empty() {
            "Results".into()
        } else {
            format!("Results (1-{}/{})", self.movies.len(), self.total).into()
        }
    }

    fn selected_overview(&self) -> Option<&str> {
        self.movies.get(self.list_state.selected()?).map(|m| match m.overview.as_str() {
            "" => "<No overview>",
            s => s,
        })
    }

    fn start_search(&mut self, query: &str, client: &mut TmdbClient) -> bool {
        if self.loading.is_some() || query == self.query {
            return false;
        }
        self.movies.clear();
        self.next_page = 0;
        self.total = 0;
        query.clone_into(&mut self.query);
        *self.list_state.offset_mut() = 0;
        *self.list_state.selected_mut() = Some(0);
        self.loading = Some(client.search_movie(self.query.clone(), 1));
        true
    }

    fn choose(&mut self, client: &mut TmdbClient) {
        if self.loading.is_none()
            && let Some(s) = self.list_state.selected()
            && s >= self.movies.len()
        {
            self.loading = Some(client.search_movie(self.query.clone(), self.next_page));
        }
    }

    fn update_results(&mut self, client: &mut TmdbClient) {
        if let Some(id) = self.loading
            && let Some(results) = client.search_movie_results(id)
        {
            let results = results.unwrap();
            self.movies.extend(results.results);
            self.next_page = results.page + 1;
            self.total = results.total_results.try_into().unwrap();
            self.loading = None;
        }
    }
}

struct Details {
    half_view: usize,
    max_offset: usize,
    list_state: ListState,
}

impl Details {
    fn new() -> Self { Self { half_view: 0, max_offset: 0, list_state: ListState::default() } }

    fn reset_offset(&mut self) { *self.list_state.offset_mut() = 0; }

    fn change_offset(&mut self, direction: ScrollDirection, offset: ScrollOffset) {
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

    fn list_items_and_update<'a>(&mut self, area: Rect, overview: &'a str) -> Vec<Cow<'a, str>> {
        let items = textwrap::wrap(overview, area.width as usize);
        self.half_view = area.height as usize / 2;
        self.max_offset = items.len().saturating_sub(area.height as _);
        let offset = self.list_state.offset_mut();
        *offset = self.max_offset.min(*offset);
        items
    }
}

impl AddMovie {
    fn new() -> Self {
        let current_window = AddMovieWindow::Search;
        let search = String::new();
        let results = Results::new();
        let details = Details::new();
        Self { current_window, search, results, details }
    }

    fn select_window(&mut self, window: AddMovieWindow) { self.current_window = window; }

    fn search(&mut self, client: &mut TmdbClient) {
        if self.results.start_search(&self.search, client) {
            self.select_window(AddMovieWindow::Results);
        }
    }

    fn change_selection(&mut self, direction: ScrollDirection) {
        if self.results.change_selected(direction) {
            self.details.reset_offset();
        }
    }

    fn scroll_preview(&mut self, direction: ScrollDirection, offset: ScrollOffset) {
        self.details.change_offset(direction, offset);
    }

    fn choose(&mut self, client: &mut TmdbClient) { self.results.choose(client); }

    fn handle_key(&mut self, client: &mut TmdbClient, event: KeyEvent) -> AppState {
        match self.current_window {
            AddMovieWindow::Search => match event.code {
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    return AppState::List,
                KeyCode::Esc => return AppState::List,
                KeyCode::Char('k') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.select_window(AddMovieWindow::Results),
                KeyCode::Enter => self.search(client),
                KeyCode::Backspace => _ = self.search.pop(),
                KeyCode::Char(c) => self.search.push(c),
                _ => {},
            },
            AddMovieWindow::Results => match event.code {
                KeyCode::Char('q') => return AppState::List,
                KeyCode::Char('j') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.select_window(AddMovieWindow::Search),
                KeyCode::Char('j') => self.change_selection(ScrollDirection::Down),
                KeyCode::Char('k') => self.change_selection(ScrollDirection::Up),
                KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.scroll_preview(ScrollDirection::Down, ScrollOffset::HalfView),
                KeyCode::Char('u') if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    self.scroll_preview(ScrollDirection::Up, ScrollOffset::HalfView),
                KeyCode::Char('J') => self.scroll_preview(ScrollDirection::Down, ScrollOffset::One),
                KeyCode::Char('K') => self.scroll_preview(ScrollDirection::Up, ScrollOffset::One),
                KeyCode::Enter => self.choose(client),
                _ => {},
            },
        }
        AppState::AddMovie
    }

    fn update(&mut self, client: &mut TmdbClient) { self.results.update_results(client); }

    fn draw(&mut self, app_state: AppState, frame: &mut Frame) {
        if app_state != AppState::AddMovie {
            return;
        }

        let popup_area = Layout::horizontal([1, 7, 1].map(Constraint::Fill))
            .split(Layout::vertical([1, 5, 1].map(Constraint::Fill)).split(frame.area())[1])[1];
        let [title_area, results_area, search_block_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1), Constraint::Length(3)])
                .spacing(Spacing::Overlap(1))
                .areas(popup_area);
        let [list_block_area, details_block_area] =
            Layout::horizontal([2, 3].map(Constraint::Fill))
                .spacing(Spacing::Overlap(1))
                .areas(results_area);

        frame.render_widget(Clear, popup_area);

        let title = Paragraph::new("Add movie").bold().centered().block(
            Block::bordered().style(Style::default().not_bold()).border_type(BorderType::Rounded),
        );
        frame.render_widget(title, title_area);

        let list_block =
            Block::bordered().merge_borders(MergeStrategy::Fuzzy).title(self.results.list_title());
        let list_inner_area = list_block.inner(list_block_area);
        frame.render_widget(list_block, list_block_area);

        let list_style = match self.current_window {
            AddMovieWindow::Search => Style::default(),
            AddMovieWindow::Results => Style::default().fg(Color::Gray),
        };
        let list = List::new(self.results.list_items(list_inner_area.width as _))
            .scroll_padding(3)
            .style(list_style)
            .highlight_style(list_style.reversed());
        frame.render_stateful_widget(list, list_inner_area, &mut self.results.list_state);

        let details_block = Block::bordered().merge_borders(MergeStrategy::Fuzzy).title("Details");
        let details_inner_area = details_block.inner(details_block_area);
        frame.render_widget(details_block, details_block_area);

        if let Some(overview) = self.results.selected_overview() {
            let details =
                List::new(self.details.list_items_and_update(details_inner_area, overview));
            frame.render_stateful_widget(details, details_inner_area, &mut self.details.list_state);
        }

        let search_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy)
            .title("Search");
        let search_text_area = search_block.inner(search_block_area);
        frame.render_widget(search_block, search_block_area);

        let search = Paragraph::new(self.search.as_str());
        if self.current_window == AddMovieWindow::Search {
            frame.set_cursor_position(
                search_text_area.as_position()
                    + Offset::new(i32::try_from(self.search.len()).unwrap(), 0),
            );
        }

        frame.render_widget(search, search_text_area);
    }
}

impl App {
    pub fn new() -> Result<Self> {
        let state = AppState::List;
        let watch_list = WatchList::new()?;
        let add_movie = AddMovie::new();
        let client = TmdbClient::new();
        Ok(Self { state, watch_list, add_movie, client })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            if self.state == AppState::Quit {
                return Ok(());
            }

            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(50))?
                && let Event::Key(k) = event::read()?
            {
                self.state = match self.state {
                    AppState::List => self.watch_list.handle_key(k),
                    AppState::AddMovie => self.add_movie.handle_key(&mut self.client, k),
                    AppState::Quit => unreachable!(),
                };
            }

            self.add_movie.update(&mut self.client);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.watch_list.draw(self.state, frame);
        self.add_movie.draw(self.state, frame);
    }
}
