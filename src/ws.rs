use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{SinkExt, Stream, StreamExt};
use riot_local_auth::Credentials;
use rustls::{ClientConfig, RootCertStore};
use rustls_pemfile::Item;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async_tls_with_config, tungstenite, tungstenite::client::IntoClientRequest,
    tungstenite::http::HeaderValue, tungstenite::Message, Connector, MaybeTlsStream,
    WebSocketStream,
};

use crate::{
    error::LcuWebsocketError,
    model::ws::{LcuEvent, LcuSubscriptionType},
};

/// A client for the League-Client(LCU) websocket API
pub struct LcuWebsocketClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl LcuWebsocketClient {
    /// Tries to establish a connection to the LCU Websocket API \
    /// Returns an [LcuWebsocketError] if the API is not reachable
    pub async fn connect() -> Result<Self, LcuWebsocketError> {
        let credentials = riot_local_auth::lcu::try_get_credentials()
            .map_err(|e| LcuWebsocketError::LcuNotAvailable(e.to_string()))?;

        Self::connect_with(&credentials).await
    }

    /// Tries to establish a connection to the LCU Websocket API \
    /// Returns an [LcuWebsocketError] if the API is not reachable
    pub async fn connect_with(credentials: &Credentials) -> Result<Self, LcuWebsocketError> {
        let mut url = format!("wss://127.0.0.1:{}", credentials.port)
            .into_client_request()
            .map_err(|_| LcuWebsocketError::AuthError)?;
        url.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&credentials.basic_auth())
                .map_err(|_| LcuWebsocketError::AuthError)?,
        );

        let mut cert_store = RootCertStore::empty();
        let Ok(Some((Item::X509Certificate(cert), _))) =
            rustls_pemfile::read_one_from_slice(include_bytes!("../riotgames.pem").as_slice())
        else {
            return Err(LcuWebsocketError::AuthError);
        };
        cert_store
            .add(cert)
            .map_err(|_| LcuWebsocketError::AuthError)?;

        let connector = Connector::Rustls(
            ClientConfig::builder()
                .with_root_certificates(cert_store)
                .with_no_client_auth()
                .into(),
        );
        let (ws_stream, _) = connect_async_tls_with_config(url, None, false, Some(connector))
            .await
            .map_err(|e| LcuWebsocketError::Disconnected(e.to_string()))?;

        Ok(Self(ws_stream))
    }

    /// The Websocket events to subscribe to.
    /// Look at the in-official documentation for event strings to subscribe to.
    ///
    /// <https://www.mingweisamuel.com/lcu-schema/tool/#/>
    pub async fn subscribe(
        &mut self,
        subscription: LcuSubscriptionType,
    ) -> Result<(), LcuWebsocketError> {
        self.0
            .send(Message::text(format!("[5, \"{subscription}\"]")))
            .await
            .map_err(|e| match e {
                tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                    LcuWebsocketError::Disconnected(e.to_string())
                }
                _ => LcuWebsocketError::SendError,
            })
    }

    /// The Websocket events to subscribe to.
    /// Look at the in-official documentation for event strings to subscribe to.
    ///
    /// <https://www.mingweisamuel.com/lcu-schema/tool/#/>
    pub async fn unsubscribe(
        &mut self,
        subscription: LcuSubscriptionType,
    ) -> Result<(), LcuWebsocketError> {
        self.0
            .send(Message::text(format!("[6, \"{subscription}\"]")))
            .await
            .map_err(|e| match e {
                tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                    LcuWebsocketError::Disconnected(e.to_string())
                }
                _ => LcuWebsocketError::SendError,
            })
    }
}

impl Stream for LcuWebsocketClient {
    type Item = LcuEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            return match self.0.poll_next_unpin(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Some(Ok(Message::Text(text)))) => {
                    let Ok(event) = serde_json::from_str::<LcuEvent>(&text) else {
                        continue;
                    };
                    Poll::Ready(Some(event))
                }
                Poll::Ready(Some(Ok(Message::Close(_))) | Some(Err(_)) | None) => Poll::Ready(None),
                _ => continue,
            };
        }
    }
}
