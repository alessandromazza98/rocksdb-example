use serde::{Deserialize, Serialize};

/// A small note identified by a unique number id.
#[derive(Deserialize, Serialize, Debug)]
pub struct Note {
    id: u64,
    content: String,
}

impl Note {
    pub fn new(id: u64, content: String) -> Self {
        Self { id, content }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
