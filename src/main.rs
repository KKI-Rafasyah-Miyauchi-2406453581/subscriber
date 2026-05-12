use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};
use std::time::Duration;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        // Uncommenting this section. Simulating the slow subscriber process
        std::thread::sleep(Duration::from_secs(1));
        
        println!("In Rafasyah's Computer [2406453581]. Message received: {:?}", message);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let listener = CrosstownBus::new_subscriber("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    
    _ = listener.subscribe(
        "user_created".to_owned(), 
        UserCreatedHandler {}, 
        crosstown_bus::QueueProperties { 
            auto_delete: false, 
            durable: false, 
            use_dead_letter: true,
            consume_queue_name: None
        }
    );

    println!("Subscriber is running. Press Ctrl+C to exit.");
    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}