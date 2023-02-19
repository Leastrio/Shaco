use futures_util::{SinkExt, StreamExt};
use sysinfo::{System, SystemExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::client::IntoClientRequest, tungstenite::http::HeaderValue, tungstenite::Message,
    Connector, MaybeTlsStream, WebSocketStream,
};

use crate::model::ws::{deserialize::DeEvent, Event, SubscriptionType, WebsocketError};
use crate::utils::process_info::*;

#[test]
fn test() {
    let json = r#"[
    8,
    "OnJsonApiEvent_lol-end-of-game_v1_eog-stats-block",
    {
        "data": {
            "basePoints": 0,
            "battleBoostIpEarned": 0,
            "boostIpEarned": 0,
            "others": "..."
        },
        "eventType": "Create",
        "uri": "/lol-end-of-game/v1/eog-stats-block"
    }
]"#;

    let event: DeEvent = serde_json::from_str(json).unwrap();
    let event = Event::from(event);
    println!("{:?}", event);
}

#[tokio::test]
async fn integration_test() {
    let mut ws_client = WSClient::connect().await.unwrap();
    ws_client
        .subscribe(SubscriptionType::AllJsonApiEvents)
        .await
        .unwrap();

    loop {
        match ws_client.next_event().await {
            Ok(e) => println!("Event: {:?}", e),
            Err(WebsocketError::DISCONNECTED) => break,
        };
    }
}

type Error = Box<dyn std::error::Error>;

pub struct WSClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WSClient {
    pub async fn connect() -> Result<Self, Error> {
        let mut sys = System::new_all();
        sys.refresh_processes();

        let process = find_process(&sys)?;
        let args = extract_info(process)?;
        let auth_token = encode_token(&args.0);

        let cert = native_tls::Certificate::from_pem(include_bytes!("./riotgames.pem"))?;
        let tls = native_tls::TlsConnector::builder()
            .add_root_certificate(cert)
            .build()?;
        let connector = Connector::NativeTls(tls);
        let mut url = format!("wss://127.0.0.1:{}", args.1).into_client_request()?;
        url.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(format!("Basic {}", auth_token).as_str())?,
        );

        let (ws_stream, _response) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, Some(connector)).await?;

        Ok(Self(ws_stream))
    }

    pub async fn subscribe(&mut self, event: SubscriptionType) -> Result<(), Error> {
        self.0
            .send(Message::text(format!("[5, \"{event}\"]")))
            .await?;
        Ok(())
    }

    pub async fn unsubscribe(&mut self, event: SubscriptionType) -> Result<(), Error> {
        self.0
            .send(Message::text(format!("[6, \"{event}\"]")))
            .await?;
        Ok(())
    }

    pub async fn next_event(&mut self) -> Result<Event, WebsocketError> {
        loop {
            let msg = match self.0.next().await {
                Some(Ok(Message::Text(msg))) => msg,
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => {
                    self.0
                        .close(None)
                        .await
                        .map_err(|_| WebsocketError::DISCONNECTED)?;
                    return Err(WebsocketError::DISCONNECTED);
                }
                _ => continue,
            };
            let Ok(de_event) = serde_json::from_str::<DeEvent>(&msg) else { continue };
            return Ok(Event::from(de_event));
        }
    }
}
