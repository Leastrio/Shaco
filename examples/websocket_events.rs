use futures_util::stream::StreamExt;

use shaco::{model::ws::LcuSubscriptionType, ws};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ws::LcuWebsocketClient::connect().await?;
    client
        .subscribe(LcuSubscriptionType::JsonApiEvent(
            "/lol-gameflow/v1/session".to_string(),
        ))
        .await
        .unwrap();

    while let Some(event) = client.next().await {
        println!("Event: {:#?}", event);
    }

    Ok(())
}
