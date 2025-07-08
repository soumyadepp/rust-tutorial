pub struct Message {
    topic: String,
    payload: String,
}

impl Message {
    pub fn new(topic: &str, payload: &str) -> Self {
        Self {
            topic: topic.to_string(),
            payload: payload.to_string(),
        }
    }

    pub fn print(&self) {
        println!("[{}] {}", self.topic, self.payload);
    }
}
