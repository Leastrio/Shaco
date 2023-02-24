use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

use shaco::model::ws::LcuSubscriptionType;
use shaco::ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ws::WSClient::connect().await?;
    client
        .subscribe(LcuSubscriptionType::AllJsonApiEvents)
        .await
        .unwrap();

    let read_future = client.for_each(|message| async move {
        tokio::io::stdout()
            .write_all(format!("{:?}", message).as_bytes())
            .await
            .unwrap();
    });

    read_future.await;

    Ok(())
}
