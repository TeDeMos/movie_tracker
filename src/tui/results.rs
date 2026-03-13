use {
    crate::{
        tmdb::{
            client::TmdbClient,
            model::{Paginated, SearchMovie},
        },
        tui::utils::{EventExt, KeyResult, ScrollDirection, ScrollOffset},
    },
    ratatui::{
        Frame,
        crossterm::event::{KeyCode, KeyEvent},
        layout::Rect,
        prelude::{Color, Line, Style, Stylize, Text},
        symbols::merge::MergeStrategy,
        widgets::{Block, List, ListItem, ListState},
    },
    textwrap::Options,
};

pub trait ResultItem: Sized {
    fn title(&self) -> impl AsRef<str>;
    fn overview(&self) -> Option<&str>;
    fn start_search(client: &mut TmdbClient, query: &str, page: i32) -> usize;
    fn get_results(client: &mut TmdbClient, id: usize) -> Option<Paginated<Self>>;
}

impl ResultItem for SearchMovie {
    fn title(&self) -> impl AsRef<str> {
        match self.release_date {
            Some(d) => format!("{} ({d})", self.original_title),
            None => format!("{}, (?)", self.original_title),
        }
    }

    fn overview(&self) -> Option<&str> {
        match self.overview.as_str() {
            "" => None,
            s => Some(s),
        }
    }

    fn start_search(client: &mut TmdbClient, query: &str, page: i32) -> usize {
        client.search_movie(query.to_string(), page)
    }

    fn get_results(client: &mut TmdbClient, id: usize) -> Option<Paginated<Self>> {
        client.search_movie_results(id).map(Result::unwrap)
    }
}

pub type SearchMovieResults = Results<SearchMovie>;

pub struct Results<T: ResultItem> {
    items: Vec<T>,
    next_page: i32,
    total: usize,
    query: String,
    list_state: ListState,
    loading: Option<usize>,
}

impl<T: ResultItem> Results<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_page: 0,
            total: 0,
            query: String::new(),
            list_state: ListState::default().with_selected(Some(0)),
            loading: None,
        }
    }

    pub fn overview(&self) -> Option<&str> {
        self.items.get(self.list_state.selected()?)?.overview()
    }

    pub fn draw(&mut self, active: bool, rect: Rect, frame: &mut Frame) {
        let block = Block::bordered().merge_borders(MergeStrategy::Fuzzy).title(self.title());
        frame.render_widget(&block, rect);

        let inner = block.inner(rect);
        let list_style = if active { Style::default() } else { Style::default().fg(Color::Gray) };
        let list = List::new(self.items(inner.width as _))
            .scroll_padding(3)
            .style(list_style)
            .highlight_style(list_style.reversed());
        frame.render_stateful_widget(list, inner, &mut self.list_state);
    }

    fn title(&self) -> Line<'static> {
        if self.items.is_empty() {
            "Results".into()
        } else {
            format!("Results (1-{}/{})", self.items.len(), self.total).into()
        }
    }

    fn items(&self, width: usize) -> impl Iterator<Item = impl Into<ListItem<'static>>> {
        const INDENT: usize = 5;
        const SPACES: &str = unsafe { str::from_utf8_unchecked(&[b' '; INDENT]) };
        const FILL: usize = INDENT - 2;

        let special = self.special_item();
        self.items
            .iter()
            .enumerate()
            .map(move |(i, t)| {
                Text::from(textwrap::fill(
                    t.title().as_ref(),
                    Options::new(width - FILL - 2)
                        .initial_indent(&format!("{:>FILL$}. ", i + 1))
                        .subsequent_indent(SPACES),
                ))
            })
            .chain(special)
    }

    fn special_item(&self) -> Option<Text<'static>> {
        if self.loading.is_some() {
            Some(Text::from("Loading...").fg(Color::Gray).italic().centered())
        } else if self.total == 0 {
            Some(Text::from("No results").fg(Color::Gray).italic().centered())
        } else if self.total > self.items.len() {
            Some(Text::from("[Load more]").bold().centered())
        } else {
            None
        }
    }

    fn last_item(&self) -> usize {
        let results = self.items.len();
        if self.loading.is_some() || self.total == 0 || self.total > results {
            results + 1
        } else {
            results
        }
    }

    pub fn handle_key(
        &mut self, event: KeyEvent, client: &mut TmdbClient,
    ) -> KeyResult<ResultsAction> {
        match event.code {
            KeyCode::Char('j') if event.no_modifiers() =>
                self.change_selection(ScrollDirection::Down),
            KeyCode::Char('k') if event.no_modifiers() =>
                self.change_selection(ScrollDirection::Up),
            KeyCode::Char('J') if event.shift_or_no_modifiers() => KeyResult::Action(
                ResultsAction::ScrollPreview(ScrollDirection::Down, ScrollOffset::One),
            ),
            KeyCode::Char('K') if event.shift_or_no_modifiers() => KeyResult::Action(
                ResultsAction::ScrollPreview(ScrollDirection::Up, ScrollOffset::One),
            ),
            KeyCode::Char('d') if event.control() => KeyResult::Action(
                ResultsAction::ScrollPreview(ScrollDirection::Down, ScrollOffset::HalfView),
            ),
            KeyCode::Char('u') if event.control() => KeyResult::Action(
                ResultsAction::ScrollPreview(ScrollDirection::Up, ScrollOffset::HalfView),
            ),
            KeyCode::Enter => self.select(client),
            _ => KeyResult::Propagate(event),
        }
    }

    fn change_selection(&mut self, direction: ScrollDirection) -> KeyResult<ResultsAction> {
        let selected = self.list_state.selected().unwrap_or_default();
        let next = match direction {
            ScrollDirection::Up => selected.saturating_sub(1),
            ScrollDirection::Down => self.last_item().min(selected + 1),
        };
        if selected == next {
            KeyResult::Consumed
        } else {
            self.list_state.select(Some(next));
            KeyResult::Action(ResultsAction::ResetPreview)
        }
    }

    fn select(&mut self, client: &mut TmdbClient) -> KeyResult<ResultsAction> {
        if self.loading.is_none()
            && let Some(s) = self.list_state.selected()
            && s >= self.items.len()
        {
            self.loading = Some(client.search_movie(self.query.clone(), self.next_page));
            KeyResult::Consumed
        } else {
            KeyResult::Action(ResultsAction::Select)
        }
    }

    pub fn search(&mut self, query: &str, client: &mut TmdbClient) -> bool {
        if self.loading.is_some() || query == self.query {
            return false;
        }
        self.items.clear();
        self.next_page = 0;
        self.total = 0;
        query.clone_into(&mut self.query);
        *self.list_state.offset_mut() = 0;
        self.list_state.select(Some(0));
        self.loading = Some(T::start_search(client, query, 1));
        true
    }

    pub fn handle_client(&mut self, client: &mut TmdbClient) {
        if let Some(id) = self.loading
            && let Some(results) = T::get_results(client, id)
        {
            self.items.extend(results.results);
            self.next_page = results.page + 1;
            self.total = results.total_results.try_into().unwrap();
            self.loading = None;
        }
    }
}

pub enum ResultsAction {
    ScrollPreview(ScrollDirection, ScrollOffset),
    ResetPreview,
    Select,
}
