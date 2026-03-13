use ratatui::crossterm::event::{KeyEvent, KeyModifiers};

pub enum KeyResult<T> {
    Consumed,
    Action(T),
    Propagate(KeyEvent),
}

impl<T> KeyResult<T> {
    pub fn handle_action<U>(self, f: impl FnOnce(T) -> Option<U>) -> KeyResult<U> {
        match self {
            KeyResult::Consumed => KeyResult::Consumed,
            KeyResult::Action(action) => match f(action) {
                Some(u) => KeyResult::Action(u),
                None => KeyResult::Consumed,
            },
            KeyResult::Propagate(event) => KeyResult::Propagate(event),
        }
    }

    pub fn handle_propagate(self, f: impl FnOnce(KeyEvent) -> Self) -> Self {
        match self {
            KeyResult::Propagate(event) => f(event),
            _ => self,
        }
    }

    pub fn into_action(self) -> Option<T> {
        match self {
            Self::Action(t) => Some(t),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ScrollOffset {
    One,
    HalfView,
}

pub trait EventExt {
    fn no_modifiers(self) -> bool;
    fn shift_or_no_modifiers(self) -> bool;
    fn control(self) -> bool;
}

impl EventExt for KeyEvent {
    fn no_modifiers(self) -> bool { self.modifiers.is_empty() }

    fn shift_or_no_modifiers(self) -> bool { !self.modifiers.intersects(!KeyModifiers::SHIFT) }

    fn control(self) -> bool { self.modifiers.contains(KeyModifiers::CONTROL) }
}
