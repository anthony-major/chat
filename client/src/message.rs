use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter, Result};

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

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{}: {}", self.username, self.content))
    }
}
