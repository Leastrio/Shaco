use reqwest::{header, Certificate};

pub fn build_reqwest_client(
    auth_token: Option<String>,
) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let cert = Certificate::from_pem(include_bytes!("../riotgames.pem"))?;
    let mut headers = header::HeaderMap::new();

    if let Some(token) = auth_token {
        let auth_header =
            header::HeaderValue::from_str(format!("Basic {}", token).as_str()).unwrap();
        headers.insert("Authorization", auth_header);
    }

    Ok(reqwest::ClientBuilder::new()
        .add_root_certificate(cert)
        .default_headers(headers)
        .build()?)
}
