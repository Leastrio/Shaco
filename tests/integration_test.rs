use futures_util::StreamExt;

use shaco::ingame::EventStream;
use shaco::model::ws::SubscriptionType;
use shaco::ws::WSClient;

#[tokio::test]
async fn ingame_event_stream() {
    let mut event_stream = EventStream::new(None).unwrap();
    while let Some(event) = event_stream.next().await {
        println!("{:?}", event)
    }
}

#[tokio::test]
async fn lcu_event_stream() {
    let mut ws_client = WSClient::connect().await.unwrap();
    ws_client
        .subscribe(SubscriptionType::AllJsonApiEvents)
        .await
        .unwrap();

    while let Some(event) = ws_client.next().await {
        println!("Event: {:?}", event);
    }
}
