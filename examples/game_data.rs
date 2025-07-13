#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = shaco::ingame::IngameClient::new();

    let data = client.all_game_data(None).await?;

    println!("{data:#?}");

    Ok(())
}
