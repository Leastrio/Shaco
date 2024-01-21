use futures_util::StreamExt;

use shaco::{ingame::EventStream, ingame::IngameClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingame_client = IngameClient::new();
    let mut event_stream = EventStream::from_ingame_client(ingame_client, None);

    while let Some(event) = event_stream.next().await {
        println!("{:?}", event)
    }

    Ok(())
}
