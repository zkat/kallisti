use std::{cell::RefCell, ops::Deref, sync::Mutex};

use kallisti::Kallisti;

#[derive(Debug, Default)]
pub struct AppStateWrapper(Mutex<RefCell<Kallisti>>);

impl Deref for AppStateWrapper {
    type Target = Mutex<RefCell<Kallisti>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AppStateWrapper {
    pub fn new() -> AppStateWrapper {
        Default::default()
    }
}
