use std::collections::HashMap;

use matrix_sdk::{ruma::UserId, Client};

#[derive(Debug, Clone, Default)]
pub struct Kallisti {
    clients: HashMap<UserId, Client>,
}

impl Kallisti {
    pub fn insert_client(&mut self, user_id: UserId, client: Client) {
        self.clients.insert(user_id, client);
    }

    pub fn get_client(&self, user_id: &UserId) -> Option<&Client> {
        self.clients.get(user_id)
    }
}
