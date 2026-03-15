use {
    crate::tui::{
        AppAction, Context,
        popup::paginated_search::{
            details::Details,
            results::{Results, ResultsAction},
            search::Search,
            search_type::SearchType,
        },
        title::Title,
        utils::{EventExt, KeyResult},
    },
    anyhow::Result,
    ratatui::{
        Frame,
        crossterm::event::{KeyCode, KeyEvent},
        layout::{Constraint, Layout, Rect, Spacing},
    },
};

mod details;
mod results;
mod search;
mod search_type;

pub struct PaginatedSearch<T: SearchType> {
    current_window: Window,
    search: Search,
    results: Results<T>,
    details: Details,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Window {
    Search,
    Results,
}

impl<T: SearchType> PaginatedSearch<T> {
    pub fn new() -> Self {
        let current_window = Window::Search;
        let search = Search::new();
        let results = Results::new();
        let details = Details::new();
        Self { current_window, search, results, details }
    }

    pub fn handle_key(&mut self, event: KeyEvent, context: Context) -> KeyResult<AppAction> {
        match self.current_window {
            Window::Search => self
                .search
                .handle_key(event)
                .on_action(|_| {
                    if self.results.search(self.search.query(), context) {
                        self.current_window = Window::Results;
                    }
                })
                .or_handle_key_with(|event| match event.code {
                    KeyCode::Char('k') if event.control() => {
                        self.current_window = Window::Results;
                        KeyResult::Consumed
                    },
                    _ => event.into(),
                }),
            Window::Results => self
                .results
                .handle_key(event, context)
                .and_then_action(|action| match action {
                    ResultsAction::ScrollPreview(d, o) => {
                        self.details.change_offset(d, o);
                        None
                    },
                    ResultsAction::ResetPreview => {
                        self.details.reset_offset();
                        None
                    },
                    ResultsAction::Select(popup) => Some(popup.into()),
                })
                .or_handle_key(|event| match event.code {
                    KeyCode::Char('j') if event.control() => {
                        self.current_window = Window::Search;
                        KeyResult::Consumed
                    },
                    _ => event.into(),
                }),
        }
    }

    pub fn handle_client(&mut self, context: Context) -> Result<()> {
        self.results.handle_client(context)
    }

    pub fn draw(&mut self, rect: Rect, frame: &mut Frame) {
        let [title, results_details, search] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1), Constraint::Length(3)])
                .spacing(Spacing::Overlap(1))
                .areas(rect);
        let [results, details] = Layout::horizontal([2, 3].map(Constraint::Fill))
            .spacing(Spacing::Overlap(1))
            .areas(results_details);

        Title("Add movie").draw(title, frame);
        self.results.draw(self.current_window == Window::Results, results, frame);
        self.details.draw(self.results.selected(), details, frame);
        self.search.draw(self.current_window == Window::Search, search, frame);
    }
}
