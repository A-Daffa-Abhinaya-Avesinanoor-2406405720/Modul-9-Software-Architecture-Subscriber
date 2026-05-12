use std::{error::Error, thread};

use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler, QueueProperties};

const AMQP_URL: &str = "amqp://guest:guest@localhost:5672";
const USER_CREATED_QUEUE: &str = "user_created";

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn get_handler_action(&self) -> String {
        "user_created_handler".to_owned()
    }

    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!(
            "In Dab's Computer 2406405720. Message received: {:?}",
            message
        );
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = CrosstownBus::new_queue_listener(AMQP_URL.to_owned())?;

    listener.listen(
        USER_CREATED_QUEUE.to_owned(),
        UserCreatedHandler,
        QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
        },
    )?;

    loop {
        thread::park();
    }
}
