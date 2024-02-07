use futures_util::StreamExt;

use shaco::ingame::EventStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_stream = EventStream::new();

    while let Some(event) = event_stream.next().await {
        println!("{:?}", event)
    }

    Ok(())
}
