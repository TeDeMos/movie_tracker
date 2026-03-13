use {
    crate::{
        db::Database,
        tmdb::client::TmdbClient,
        tui::{
            main_view::MainView,
            popup::Popup,
            utils::{EventExt, KeyResult},
        },
    },
    anyhow::Result,
    ratatui::{
        DefaultTerminal, Frame,
        crossterm::event::{self, KeyCode, KeyEvent},
    },
    std::time::Duration,
};

mod details;
mod main_view;
mod popup;
mod results;
mod search;
mod title;
mod utils;

pub struct App {
    main_view: MainView,
    popup: Option<Popup>,
    database: Database,
    client: TmdbClient,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            main_view: MainView::new()?,
            popup: None,
            database: Database::new()?,
            client: TmdbClient::new(),
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Some(event) = Self::get_key_event()?
                && let Some(action) = self.handle_key(event)
                && self.handle_action(action)
            {
                return Ok(());
            }

            self.handle_client();
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        match &mut self.popup {
            Some(popup) => {
                self.main_view.draw(true, frame);
                popup.draw(frame);
            },
            None => self.main_view.draw(false, frame),
        }
    }

    fn handle_key(&mut self, event: KeyEvent) -> Option<AppAction> {
        let result = match &mut self.popup {
            Some(popup) => popup.handle_key(event, &mut self.client),
            None => self.main_view.handle_key(event),
        };
        result
            .handle_propagate(|e| match e.code {
                KeyCode::Char('q') if event.no_modifiers() => KeyResult::Action(AppAction::Quit),
                KeyCode::Char('c') if event.control() => KeyResult::Action(AppAction::Quit),
                _ => KeyResult::Consumed,
            })
            .into_action()
    }

    fn handle_action(&mut self, action: AppAction) -> bool {
        match action {
            AppAction::ShowPopup(p) => self.popup = Some(p),
            AppAction::HidePopup => self.popup = None,
            AppAction::Quit => return true,
        }
        false
    }

    fn handle_client(&mut self) {
        match &mut self.popup {
            Some(popup) => popup.handle_client(&mut self.client),
            None => self.main_view.handle_client(&mut self.client),
        }
    }

    fn get_key_event() -> Result<Option<KeyEvent>> {
        if event::poll(Duration::from_millis(16))? {
            Ok(event::read()?.as_key_event())
        } else {
            Ok(None)
        }
    }
}

enum AppAction {
    ShowPopup(Popup),
    HidePopup,
    Quit,
}
