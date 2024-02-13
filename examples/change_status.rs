#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = shaco::rest::RESTClient::new()?;

    println!(
        "{:#?}",
        client
            .get("/lol-gameflow/v1/gameflow-metadata/player-status")
            .await?
    );

    Ok(())
}
