use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    username: String,
    content: String,
}

impl Message {
    pub fn new(username: String, content: String) -> Self {
        Self {
            username: username,
            content: content,
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}
