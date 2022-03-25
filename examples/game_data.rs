use shaco;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = shaco::InGameClient::new()?;

    let data = client.all_game_data().await?;

    println!("{:#?}", data);

    Ok(())
}
