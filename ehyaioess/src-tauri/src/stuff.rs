struct Message {
    sender: String,
    content: String,
}
struct State {
    message_history: Vec<Message>,
}
impl State {
    // Add methods here
    fn new() -> Self {
        Self {
            message_history: Vec::new(),
        }
    }

    fn add_message(&mut self, message: Message) {
        self.message_history.push(message);
    }

    fn get_history(&self) -> &Vec<Message> {
        &self.message_history
    }
}
