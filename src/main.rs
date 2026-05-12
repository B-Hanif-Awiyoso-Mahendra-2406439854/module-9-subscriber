use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};
use std::{env, thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let _ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();

        std::thread::sleep(_ten_millis);

        println!(
            "In Hanif's Computer [2406439854]. Message received: {:?}. Processed in {:?}",
            message,
            now.elapsed()
        );
        Ok(())
    }
}

fn main() {
    let amqp_url =
        env::var("AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_owned());
    let listener = loop {
        match CrosstownBus::new_queue_listener(amqp_url.clone()) {
            Ok(listener) => break listener,
            Err(error) => {
                println!("Waiting for RabbitMQ at {amqp_url}: {error:?}");
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    };

    _ = listener.listen(
        "user_created".to_owned(),
        UserCreatedHandler {},
        crosstown_bus::QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
        },
    );

    loop {}
}
