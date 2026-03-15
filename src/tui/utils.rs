use ratatui::crossterm::event::{KeyEvent, KeyModifiers};

pub enum KeyResult<T> {
    Consumed,
    Action(T),
    Propagate(KeyEvent),
}

impl<T> KeyResult<T> {
    pub fn and_then_action<U>(self, f: impl FnOnce(T) -> Option<U>) -> KeyResult<U> {
        match self {
            Self::Consumed => KeyResult::Consumed,
            Self::Action(action) => match f(action) {
                Some(u) => KeyResult::Action(u),
                None => KeyResult::Consumed,
            },
            Self::Propagate(event) => KeyResult::Propagate(event),
        }
    }

    pub fn on_action(self, f: impl FnOnce(T)) -> KeyResult<!> {
        match self {
            Self::Consumed => KeyResult::Consumed,
            Self::Action(action) => {
                f(action);
                KeyResult::Consumed
            },
            Self::Propagate(event) => KeyResult::Propagate(event),
        }
    }

    pub fn or_handle_key(self, f: impl FnOnce(KeyEvent) -> Self) -> Self {
        match self {
            Self::Consumed => Self::Consumed,
            Self::Action(t) => Self::Action(t),
            Self::Propagate(event) => f(event),
        }
    }

    pub fn into_action(self) -> Option<T> {
        match self {
            Self::Action(t) => Some(t),
            _ => None,
        }
    }
}

impl KeyResult<!> {
    pub fn or_handle_key_with<T>(self, f: impl FnOnce(KeyEvent) -> KeyResult<T>) -> KeyResult<T> {
        match self {
            Self::Consumed => KeyResult::Consumed,
            Self::Action(n) => n,
            Self::Propagate(event) => f(event),
        }
    }
}

impl<T> From<KeyEvent> for KeyResult<T> {
    fn from(value: KeyEvent) -> Self { Self::Propagate(value) }
}

pub trait IntoAction: Sized {
    fn action(self) -> KeyResult<Self>;
}

impl<T> IntoAction for T {
    fn action(self) -> KeyResult<T> { KeyResult::Action(self) }
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
