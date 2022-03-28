use futures_util::{SinkExt, StreamExt};
use sysinfo::{System, SystemExt};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

use crate::utils::process_info::*;
use crate::WSClient;

type Error = Box<dyn std::error::Error>;

#[derive(PartialEq)]
pub enum Events {
    Json,
    Lcds,
    Callback,
    None,
}

impl WSClient {
    pub async fn connect(events: Events) -> Result<Self, Error> {
        let mut sys = System::new_all();
        sys.refresh_all();
        let process = find_process(&sys)?;
        let args = extract_info(process)?;
        let auth_token = encode_token(&args.0);

        let cert = native_tls::Certificate::from_pem(include_bytes!("./riotgames.pem"))?;
        let tls = native_tls::TlsConnector::builder()
            .add_root_certificate(cert)
            .build()?;
        let connector = tokio_tungstenite::Connector::NativeTls(tls);
        let mut url = format!("wss://127.0.0.1:{}", args.1).into_client_request()?;
        {
            let headers = url.headers_mut();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(format!("Basic {}", auth_token).as_str())?,
            );
        }

        let (ws_stream, _response) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, Some(connector)).await?;

        let (mut write, read) = ws_stream.split();

        match events {
            Events::Json => write.send(Message::text("[5, \"OnJsonApiEvent\"]")).await?,
            Events::Lcds => write.send(Message::text("[5, \"OnLcdsEvent\"]")).await?,
            Events::Callback => write.send(Message::text("[5, \"OnCallback\"]")).await?,
            Events::None => (),
        }

        Ok(Self { write, read })
    }

    pub async fn subscribe(&mut self, event: String) -> Result<(), Error> {
        self.write
            .send(Message::text(format!("[5, {}]", event)))
            .await?;

        Ok(())
    }

    pub async fn unsubscribe(&mut self, event: String) -> Result<(), Error> {
        self.write
            .send(Message::text(format!("[6, {}]", event)))
            .await?;

        Ok(())
    }
}
