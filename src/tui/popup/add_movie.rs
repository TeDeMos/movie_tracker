use {
    crate::{
        tmdb::client::TmdbClient,
        tui::{
            AppAction, KeyResult,
            details::Details,
            results::{ResultsAction, SearchMovieResults},
            search::Search,
            title::Title,
            utils::EventExt,
        },
    },
    ratatui::{
        Frame,
        crossterm::event::{KeyCode, KeyEvent},
        layout::{Constraint, Layout, Rect, Spacing},
    },
};

pub struct AddMovie {
    current_window: AddMovieWindow,
    search: Search,
    results: SearchMovieResults,
    details: Details,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum AddMovieWindow {
    Search,
    Results,
}

impl AddMovie {
    pub fn new() -> Self {
        let current_window = AddMovieWindow::Search;
        let search = Search::new();
        let results = SearchMovieResults::new();
        let details = Details::new();
        Self { current_window, search, results, details }
    }

    pub fn handle_key(&mut self, event: KeyEvent, client: &mut TmdbClient) -> KeyResult<AppAction> {
        match self.current_window {
            AddMovieWindow::Search => self
                .search
                .handle_key(event)
                .handle_action(|_| {
                    if self.results.search(self.search.query(), client) {
                        self.current_window = AddMovieWindow::Results;
                    }
                    None
                })
                .handle_propagate(|event| match event.code {
                    KeyCode::Char('k') if event.control() => {
                        self.current_window = AddMovieWindow::Results;
                        KeyResult::Consumed
                    },
                    _ => KeyResult::Propagate(event),
                }),
            AddMovieWindow::Results => self
                .results
                .handle_key(event, client)
                .handle_action(|action| match action {
                    ResultsAction::ScrollPreview(d, o) => {
                        self.details.change_offset(d, o);
                        None
                    },
                    ResultsAction::ResetPreview => {
                        self.details.reset_offset();
                        None
                    },
                    ResultsAction::Select => Some(AppAction::HidePopup),
                })
                .handle_propagate(|event| match event.code {
                    KeyCode::Char('j') if event.control() => {
                        self.current_window = AddMovieWindow::Search;
                        KeyResult::Consumed
                    },
                    _ => KeyResult::Propagate(event),
                }),
        }
    }

    pub fn handle_client(&mut self, client: &mut TmdbClient) { self.results.handle_client(client); }

    pub fn draw(&mut self, rect: Rect, frame: &mut Frame) {
        let [title, results_details, search] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1), Constraint::Length(3)])
                .spacing(Spacing::Overlap(1))
                .areas(rect);
        let [results, details] = Layout::horizontal([2, 3].map(Constraint::Fill))
            .spacing(Spacing::Overlap(1))
            .areas(results_details);

        Title("Add movie").draw(title, frame);
        self.results.draw(self.current_window == AddMovieWindow::Results, results, frame);
        self.details.draw(self.results.overview(), details, frame);
        self.search.draw(self.current_window == AddMovieWindow::Search, search, frame);
    }
}
