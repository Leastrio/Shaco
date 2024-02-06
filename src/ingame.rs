use std::{task::Poll, time::Duration};

use futures_util::Stream;
use reqwest::Response;
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver},
    sync::oneshot,
    sync::oneshot::Sender,
    task::JoinHandle,
};

use crate::{error::IngameClientError, model::ingame::*, utils::request::build_reqwest_client};

const PORT: u16 = 2999;

/// A client for the LoL-Ingame API
pub struct IngameClient(reqwest::Client);

impl IngameClient {
    /// Create a new connection to the ingame api. This will return an error if a game is not running
    pub fn new() -> Result<Self, IngameClientError> {
        Ok(Self(build_reqwest_client(None)))
    }

    /// Checks if there is an active game \
    /// Returns true only after the loading screen
    pub async fn active_game(&self) -> bool {
        let req = self
            .0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataGamestats",
                PORT
            ))
            .send()
            .await;

        if let Ok(req) = req {
            // as soon as your own game has loaded the API returns GameStats but with a game_time slightly bigger than 0.0
            // this game_time stays constant until the game starts
            // as far as I can tell the value is always something around ~0.02
            // => check if game_time is > 0.1 as a proxy for if the game has started
            if let Ok(game_stats) = req.json::<GameStats>().await {
                return game_stats.game_time > 0.1;
            }
        }

        false
    }

    /// Checks if there is an active game \
    /// Same as [`IngameClient::active_game`] but returns true during loading screen when other API calls still return Error.
    /// Also returns true when the game has already started.
    pub async fn active_game_loadingscreen(&self) -> bool {
        let req = self
            .0
            .head(format!("https://127.0.0.1:{}/Help", PORT))
            .send()
            .await;

        if let Ok(req) = req {
            req.status().is_success()
        } else {
            false
        }
    }

    /// Checks if the game is a livegame or in spectatormode
    pub async fn is_spectator_mode(&self) -> Result<bool, IngameClientError> {
        // before the game has actually started the API sometimes returns falsely
        // that the game is a spectator game when it is not
        if !self.active_game().await {
            return Err(IngameClientError::ApiNotAvailableDuringLoadingScreen);
        }

        let req = self
            .0
            .head(format!(
                "https://127.0.0.1:{}/GetLiveclientdataActiveplayer",
                PORT
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from);

        match req {
            Ok(_) => Ok(false),
            Err(IngameClientError::ApiNotAvailableInSpectatorMode) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Get all current game data
    pub async fn all_game_data(
        &self,
        event_id: Option<u32>,
    ) -> Result<AllGameData, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataAllgamedata?eventID={}",
                PORT,
                event_id.unwrap_or(0) // an event_id of 0 returns all events
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get event data for the active game
    pub async fn event_data(
        &self,
        event_id: Option<u32>,
    ) -> Result<Vec<GameEvent>, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataEventdata?eventID={}",
                PORT,
                event_id.unwrap_or(0) // an event_id of 0 returns all events
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json::<IngameEvents>()
            .await
            .map_err(IngameClientError::from)
            .map(|ie| ie.events)
    }

    /// Get the active games stats
    pub async fn game_stats(&self) -> Result<GameStats, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataGamestats",
                PORT
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get a specified players items
    pub async fn player_items<S: AsRef<str>>(
        &self,
        summoner_name: S,
    ) -> Result<Vec<PlayerItem>, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataPlayeritems?summonerName={}",
                PORT,
                summoner_name.as_ref()
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get a list of players in game
    pub async fn player_list(
        &self,
        team_id: Option<TeamId>,
    ) -> Result<Vec<Player>, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataPlayerlist?teamID={}",
                PORT,
                team_id.unwrap_or(TeamId::All)
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get a specified players main runes
    pub async fn player_main_runes<S: AsRef<str>>(
        &self,
        summoner_name: S,
    ) -> Result<PlayerRunes, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataPlayermainrunes?summonerName={}",
                PORT,
                summoner_name.as_ref()
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get a specified players score
    pub async fn player_scores<S: AsRef<str>>(
        &self,
        summoner_name: S,
    ) -> Result<PlayerScores, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataPlayerscores?summonerName={}",
                PORT,
                summoner_name.as_ref()
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get specified players summoner spells
    pub async fn player_summoner_spells<S: AsRef<str>>(
        &self,
        summoner_name: S,
    ) -> Result<SummonerSpells, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataPlayersummonerspells?summonerName={}",
                PORT,
                summoner_name.as_ref()
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get active players data \
    /// Only available during livegame
    pub async fn active_player(&self) -> Result<ActivePlayer, IngameClientError> {
        /// only available in live games - is Error when spectating
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[serde(untagged)]
        enum ActivePlayerInfo {
            ActivePlayer(Box<ActivePlayer>),
            Error { error: String },
        }

        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataActiveplayer",
                PORT
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json::<ActivePlayerInfo>()
            .await
            .map_err(IngameClientError::from)
            .map(|i| match i {
                ActivePlayerInfo::ActivePlayer(i) => Ok(*i),
                ActivePlayerInfo::Error { error } => {
                    Err(IngameClientError::DeserializationError(error))
                }
            })?
    }

    /// Get the active players abilities \
    /// Only available during livegame
    pub async fn active_player_abilities(&self) -> Result<PlayerAbilities, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataActiveplayerabilities",
                PORT
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }

    /// Get the active players name \
    /// Only available during livegame
    pub async fn active_player_name(&self) -> Result<String, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataActiveplayername",
                PORT
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .text()
            .await
            .map(|txt| {
                // remove the first and last character since the received text is wrapped in quotes (e.g. "playerName")
                let mut chars = txt.chars();
                chars.next();
                chars.next_back();
                chars.as_str().to_string()
            })
            .map_err(IngameClientError::from)
    }

    /// Get the active players runes \
    /// Only available during livegames
    pub async fn active_player_runes(&self) -> Result<FullPlayerRunes, IngameClientError> {
        self.0
            .get(format!(
                "https://127.0.0.1:{}/GetLiveclientdataActiveplayerrunes",
                PORT
            ))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(IngameClientError::from)?
            .json()
            .await
            .map_err(IngameClientError::from)
    }
}

const DEFAULT_POLLING_RATE_MILLIS: u64 = 500;

/// A wrapper around a [IngameClient] that regularly polls the ingame events
pub struct EventStream {
    start_tx: Option<Sender<()>>,
    poll_task_handle: JoinHandle<()>,
    events_rx: UnboundedReceiver<GameEvent>,
}

impl EventStream {
    /// Create an [EventStream] from an [IngameClient] \
    /// Takes an [Option] that specifies the polling rate of the [IngameClient] that's being wrapped \
    /// The default [Duration] is 500ms
    pub fn from_ingame_client(ingame_client: IngameClient, polling_rate: Option<Duration>) -> Self {
        let (start_tx, start_rx) = oneshot::channel::<()>();
        let (events_tx, events_rx) = unbounded_channel();

        let poll_task_handle = tokio::spawn(async move {
            let polling_rate =
                polling_rate.unwrap_or(Duration::from_millis(DEFAULT_POLLING_RATE_MILLIS));
            let mut timer = tokio::time::interval(polling_rate);
            let mut current_event_id = 0;

            // await start, but return on error (start_tx got dropped)
            if start_rx.await.is_err() {
                return;
            }

            // wait for a game to start
            loop {
                timer.tick().await;
                if ingame_client.active_game().await {
                    break;
                };
            }

            // loop for as long as api calls are successful
            loop {
                timer.tick().await;
                match ingame_client.event_data(Some(current_event_id)).await {
                    Ok(events) => {
                        if let Some(last_event) = events.last() {
                            current_event_id = last_event.get_event_id() + 1;
                        }
                        events.into_iter().for_each(|e| _ = events_tx.send(e))
                    }
                    Err(_) => return,
                }
            }
        });

        Self {
            start_tx: Some(start_tx),
            poll_task_handle,
            events_rx,
        }
    }
}

impl Stream for EventStream {
    type Item = GameEvent;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if let Some(start_tx) = self.start_tx.take() {
            if start_tx.send(()).is_err() {
                return Poll::Ready(None);
            }
        }
        self.events_rx.poll_recv(cx)
    }
}

impl Drop for EventStream {
    fn drop(&mut self) {
        self.poll_task_handle.abort()
    }
}
