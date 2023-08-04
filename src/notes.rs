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

pub fn notes_value_parser(s: &str) -> Result<(u64, String), &'static str> {
    // split the input string by '\n' character
    let split: Vec<&str> = s.split(' ').collect();

    if split.len() % 2 != 0 {
        return Err("Each note should have an ID and content");
    }

    //parse the id
    let id = match split[0].parse::<u64>() {
        Ok(id) => id,
        Err(_) => return Err("Failed to parse id"),
    };

    // clone the content to create an owned String
    let content = split[1].to_string();

    Ok((id, content))
}
