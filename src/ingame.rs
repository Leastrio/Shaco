use std::task::Poll;
use std::time::Duration;

use futures_util::Stream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;

use crate::model::ingame::*;
use crate::utils::request::build_reqwest_client;

pub struct InGameClient {
    port: u32,
    reqwest_client: reqwest::Client,
}

impl InGameClient {
    /// Create a new connection to the ingame api. This will return an error if a game is not detected
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = build_reqwest_client(None)?;
        Ok(Self {
            port: 2999,
            reqwest_client: client,
        })
    }

    /// Get all current game data
    pub async fn all_game_data(
        &self,
        event_id: Option<u32>,
    ) -> Result<AllGameData, reqwest::Error> {
        let parameter: String = if let Some(id) = event_id {
            format!("?eventID={}", id)
        } else {
            String::from("")
        };

        let req: AllGameData = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}{}",
                self.port, "allgamedata", parameter
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get active player's data
    pub async fn active_player(&self) -> Result<ActivePlayer, reqwest::Error> {
        let req: ActivePlayer = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "activeplayer"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get the active player's name
    pub async fn active_player_name(&self) -> Result<String, reqwest::Error> {
        let req: String = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "activeplayername"
            ))
            .send()
            .await?
            .text()
            .await?;

        Ok(req)
    }

    /// Get the active player's abilities
    pub async fn active_player_abilities(&self) -> Result<ActivePlayerAbilities, reqwest::Error> {
        let req: ActivePlayerAbilities = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "activeplayerabilities"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get the active player's runes
    pub async fn active_player_runes(&self) -> Result<ActivePlayerRunes, reqwest::Error> {
        let req: ActivePlayerRunes = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "activeplayerrunes"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get a list of players in game
    pub async fn player_list(
        &self,
        team_id: Option<TeamID>,
    ) -> Result<Vec<Player>, reqwest::Error> {
        let parameter: &str = if let Some(teams) = team_id {
            match teams {
                TeamID::ALL => "?teamID=ALL",
                TeamID::UNKNOWN => "?teamID=UNKNOWN",
                TeamID::ORDER => "?teamID=ORDER",
                TeamID::CHAOS => "?teamID=CHAOS",
                TeamID::NEUTRAL => "?teamID=NEUTRAL",
            }
        } else {
            ""
        };

        let req: Vec<Player> = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}{}",
                self.port, "playerlist", parameter
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get a specified player's score
    pub async fn player_scores(
        &self,
        summoner_name: String,
    ) -> Result<PlayerScores, reqwest::Error> {
        let req: PlayerScores = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}?summonerName={}",
                self.port, "playerscores", summoner_name
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get specified player's summoner spells
    pub async fn player_summoner_spells(
        &self,
        summoner_name: String,
    ) -> Result<PlayerSummonerSpells, reqwest::Error> {
        let req: PlayerSummonerSpells = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}?summonerName={}",
                self.port, "playersummonerspells", summoner_name
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get a specified player's main runes
    pub async fn player_main_runes(
        &self,
        summoner_name: String,
    ) -> Result<PlayerMainRunes, reqwest::Error> {
        let req: PlayerMainRunes = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}?summonerName={}",
                self.port, "playermainrunes", summoner_name
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get a specified player's items
    pub async fn player_items(
        &self,
        summoner_name: String,
    ) -> Result<Vec<PlayerItem>, reqwest::Error> {
        let req: Vec<PlayerItem> = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}?summonerName={}",
                self.port, "playeritems", summoner_name
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get event data for the active game
    pub async fn event_data(&self, event_id: Option<u32>) -> Result<EventData, reqwest::Error> {
        let parameter: String = if let Some(id) = event_id {
            format!("?eventID={}", id)
        } else {
            String::from("")
        };

        let req: EventData = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}{}",
                self.port, "eventdata", parameter
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Get the active game's stats
    pub async fn game_stats(&self) -> Result<GameStats, reqwest::Error> {
        let req: GameStats = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "gamestats"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    pub async fn active_game(&self) -> bool {
        let req = self
            .reqwest_client
            .head(format!("https://127.0.0.1:{}/Help", self.port)) // only /Help doesn't 404 during loading screen
            .send()
            .await;

        if let Ok(req) = req {
            req.status().is_success()
        } else {
            false
        }
    }
}

pub struct EventStream {
    start_tx: Option<Sender<()>>,
    poll_task_handle: JoinHandle<()>,
    events_rx: UnboundedReceiver<Event>,
}

impl EventStream {
    pub fn new(polling_rate: Option<Duration>) -> Result<Self, Box<dyn std::error::Error>> {
        let (start_tx, start_rx) = oneshot::channel::<()>();
        let (events_tx, events_rx) = unbounded_channel();

        let ingame_client = InGameClient::new()?;
        let poll_task_handle = tokio::spawn(async move {
            let polling_rate = polling_rate.unwrap_or(Duration::from_millis(500));
            let mut timer = tokio::time::interval(polling_rate);
            let mut current_event_id = 0;

            // await start, but return on error (start_tx got dropped)
            if let Err(_) = start_rx.await {
                return;
            }

            // wait for a game to start
            loop {
                timer.tick().await;
                if ingame_client.active_game().await {
                    break;
                };
            }

            loop {
                timer.tick().await;
                match ingame_client.event_data(Some(current_event_id)).await {
                    Ok(mut events) => {
                        if let Some(last_event) = events.Events.last() {
                            current_event_id = last_event.EventID + 1;
                        }
                        events.Events.drain(..).for_each(|e| {
                            let _ = events_tx.send(e);
                        })
                    }
                    // before all players have loaded into the game all api calls return 404
                    Err(e) if e.status().is_some() => continue,
                    _ => return,
                }
            }
        });

        Ok(Self {
            start_tx: Some(start_tx),
            poll_task_handle,
            events_rx,
        })
    }
}

impl Stream for EventStream {
    type Item = Event;

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
