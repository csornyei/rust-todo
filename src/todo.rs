use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    text: String,
    done: bool,
}

impl Todo {
    pub fn new(text: &str) -> Todo {
        Todo {
            text: text.to_string(),
            done: false,
        }
    }

    pub fn complete(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}",
            if self.done { '\u{2713}' } else { ' ' },
            self.text
        )
    }
}
