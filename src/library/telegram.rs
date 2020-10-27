use crate::library::Parser;
use futures::StreamExt;
use telegram_bot::*;

pub struct Sender {
    token: String,
}

impl Sender {
    pub fn new(token: &String) -> Self {
        Sender {
            token: token.into(),
        }
    }

    pub async fn run<T: Parser>(&self, parser: T) -> Result<(), Box<dyn std::error::Error>> {
        let api = Api::new(&self.token);
        let mut stream = api.stream();

        while let Some(update) = stream.next().await {
            let update = update.map_err(|err| err.to_string())?;

            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    println!("<{}>: {}", &message.from.first_name, data);

                    api.send(message.text_reply(format!("{}", parser.parse().await?)))
                        .await
                        .map_err(|err| err.to_string())?;
                }
            }
        }

        Ok(())
    }
}
