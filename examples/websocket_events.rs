use futures_util::StreamExt;
use shaco::ws;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ws::WSClient::connect(ws::Events::Json).await?;
    let read = client.read;

    let read_future = read.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write(&data).await.unwrap();
    });

    read_future.await;

    Ok(())
}
