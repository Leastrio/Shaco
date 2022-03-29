use shaco::ingame;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ingame::InGameClient::new()?;

    let data = client.all_game_data().await?;

    println!("{:#?}", data);

    Ok(())
}
