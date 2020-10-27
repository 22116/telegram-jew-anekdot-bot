mod library;

use crate::library::{ClassicTravel, Parser, Sender};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let sender = Sender::new(&token);

    sender.run(ClassicTravel).await
}
