use std::{cell::RefCell, ops::Deref, sync::Mutex};

pub struct AppStateWrapper(Mutex<RefCell<AppState>>);

impl Deref for AppStateWrapper {
    type Target = Mutex<RefCell<AppState>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AppState {
    pub notifications: usize,
}

impl AppState {
    pub fn new_wrapped() -> AppStateWrapper {
        AppStateWrapper(Mutex::new(RefCell::new(AppState {
            notifications: 0,
        })))
    }
}
