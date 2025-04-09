pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: &str, user: &str) -> Self {
        Message {
            content: content.to_string(),
            user: user.to_string(),
        }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message, "user1");

    match msg.send_ms() {
        Some(_) => Ok(message),
        None => Err("Error: illegal")
    }
}