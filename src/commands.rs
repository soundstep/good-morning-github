#[derive(Debug)]
pub struct Command {
    pub key: String,
    pub label: String,
}

impl Command {
    pub fn new(key: String, label: String) -> Self {
        Command { key, label }
    }
}
