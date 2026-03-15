use {
    crate::{
        db::Database,
        tmdb::{client::TmdbClient, utils::ApiResult},
        tui::{
            main_view::MainView,
            popup::Popup,
            utils::{EventExt, IntoAction, KeyResult},
        },
    },
    anyhow::Result,
    derive_more::From,
    ratatui::{
        DefaultTerminal, Frame,
        crossterm::event::{self, KeyCode, KeyEvent},
    },
    std::time::Duration,
};

mod main_view;
mod popup;
mod title;
mod utils;

pub struct App {
    main_view: MainView,
    popup: Option<Popup>,
    database: Database,
    client: TmdbClient,
}

pub struct Context<'a> {
    pub database: &'a mut Database,
    pub client: &'a mut TmdbClient,
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

            if let Err(e) = self.handle_client() {
                self.popup = Some(Popup::warning(e));
            }
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
        let context = Context { database: &mut self.database, client: &mut self.client };
        let result = match &mut self.popup {
            Some(popup) => popup.handle_key(event, context),
            None => self.main_view.handle_key(event),
        };
        result
            .or_handle_key(|e| match e.code {
                KeyCode::Char('q') if event.no_modifiers() => AppAction::Quit.action(),
                KeyCode::Char('c') if event.control() => AppAction::Quit.action(),
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

    fn handle_client(&mut self) -> Result<()> {
        let context = Context { database: &mut self.database, client: &mut self.client };
        match &mut self.popup {
            Some(popup) => popup.handle_client(context),
            None => Ok(()),
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

#[derive(From)]
enum AppAction {
    ShowPopup(Popup),
    HidePopup,
    Quit,
}
