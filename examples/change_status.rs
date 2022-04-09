use shaco::rest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rest::RESTClient::new()?;

    client.put("/lol-chat/v1/me".to_string(),
        serde_json::json!({
            "statusMessage": "Please DO NOT buy the BTS meal if you don't stan them. You're preventing the actual BTS fans who have waited for months from having the BTS meal experience. Eating the sauces without understanding their significance is literally cultural appropriation and it's not okay"
        })
    ).await?;

    println!("Status Changed!");

    Ok(())
}
