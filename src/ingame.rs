use crate::model::ingame::*;
use crate::utils::request::build_reqwest_client;

pub struct InGameClient {
    port: u32,
    reqwest_client: reqwest::Client,
}

impl InGameClient {
    /// Create a new connection to the ingame api. This will not return an error if a game is not detected
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = build_reqwest_client(None)?;
        Ok(Self {
            port: 2999,
            reqwest_client: client,
        })
    }

    /// Get all current game data
    pub async fn all_game_data(&self) -> Result<AllGameData, reqwest::Error> {
        let req: AllGameData = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "allgamedata"
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
    pub async fn player_list(&self) -> Result<Vec<Player>, reqwest::Error> {
        let req: Vec<Player> = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "playerlist"
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
    pub async fn event_data(&self) -> Result<EventData, reqwest::Error> {
        let req: EventData = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}/liveclientdata/{}",
                self.port, "eventdata"
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
}
