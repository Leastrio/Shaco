#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AllGameData {
    pub activePlayer: ActivePlayer,
    pub allPlayers: Vec<Player>,
    pub events: EventData,
    pub gameData: GameStats
}

#[derive(Debug, Deserialize)]
pub struct ActivePlayer {
    pub abilities: ActivePlayerAbilities,
    pub championStats: ChampionStats,
    pub currentGold: f64,
    pub fullRunes: ActivePlayerRunes,
    pub level: u32,
    pub summonerName: String,
    pub teamRelativeColors: bool
}

#[derive(Debug, Deserialize)]
pub struct ChampionStats {
    pub abilityHaste: f64,
    pub abilityPower: f64,
    pub armor: f64,
    pub armorPenetrationFlat: f64,
    pub armorPenetrationPercent: f64,
    pub attackDamage: f64,
    pub attackRange: f64,
    pub attackSpeed: f64,
    pub bonusArmorPenetrationPercent: f64,
    pub bonusMagicPenetrationPercent: f64,
    pub critChance: f64,
    pub critDamage: f64,
    pub currentHealth: f64,
    pub healShieldPower: f64,
    pub healthRegenRate: f64,
    pub lifeSteal: f64,
    pub magicLethality: f64,
    pub magicPenetrationFlat: f64,
    pub magicPenetrationPercent: f64,
    pub magicResist: f64,
    pub maxHealth: f64,
    pub moveSpeed: f64,
    pub omnivamp: f64,
    pub physicalLethality: f64,
    pub physicalVamp: f64,
    pub resourceMax: f64,
    pub resourceRegenRate: f64,
    pub resourceType: String,
    pub resourceValue: f64,
    pub spellVamp: f64,
    pub tenacity: f64
}

#[derive(Debug, Deserialize)]
pub struct ActivePlayerAbilities {
    pub E: PlayerAbility,
    pub Passive: PlayerAbility,
    pub Q: PlayerAbility,
    pub R: PlayerAbility,
    pub W: PlayerAbility
}

#[derive(Debug, Deserialize)]
pub struct PlayerAbility {
    pub abilityLevel: Option<u32>,
    pub displayName: String,
    pub id: String,
    pub rawDescription: String,
    pub rawDisplayName: String
}

#[derive(Debug, Deserialize)]
pub struct ActivePlayerRunes {
    pub generalRunes: Vec<PlayerRune>,
    pub keystone: PlayerRune,
    pub primaryRuneTree: PlayerRune,
    pub secondaryRuneTree: PlayerRune,
    pub statRunes: Vec<StatRune>
}

#[derive(Debug, Deserialize)]
pub struct PlayerRune {
    pub displayName: String,
    pub id: u32,
    pub rawDescription: String,
    pub rawDisplayName: String
}

#[derive(Debug, Deserialize)]
pub struct StatRune {
    pub id: u32,
    pub rawDescription: String
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub championName: String,
    pub isBot: bool,
    pub isDead: bool,
    pub items: Vec<PlayerItem>,
    pub level: u32,
    pub position: String,
    pub rawChampionName: String,
    pub rawSkinName: Option<String>,
    pub respawnTimer: f64,
    pub runes: PlayerMainRunes,
    pub scores: PlayerScores,
    pub skinID: u32,
    pub skinName: Option<String>,
    pub summonerName: String,
    pub summonerSpells: PlayerSummonerSpells,
    pub team: String
}

#[derive(Debug, Deserialize)]
pub struct PlayerScores {
    pub assists: u32,
    pub creepScore: u32,
    pub deaths: u32,
    pub kills: u32,
    pub wardScore: f64
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummonerSpells {
    pub summonerSpellOne: SummonerSpell,
    pub summonerSpellTwo: SummonerSpell
}

#[derive(Debug, Deserialize)]
pub struct SummonerSpell {
    pub displayName: String,
    pub rawDescription: String,
    pub rawDisplayName: String
}

#[derive(Debug, Deserialize)]
pub struct PlayerMainRunes {
    pub keystone: PlayerRune,
    pub primaryRuneTree: PlayerRune,
    pub secondaryRuneTree: PlayerRune
}

#[derive(Debug, Deserialize)]
pub struct PlayerItem {
    pub canUse: bool,
    pub consumable: bool,
    pub count: u32,
    pub displayName: String,
    pub itemID: u32,
    pub price: u32,
    pub rawDescription: String,
    pub rawDisplayName: String,
    pub slot: u32
}

#[derive(Debug, Deserialize)]
pub struct EventData {
    pub Events: Vec<Event>
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub EventID: u32,
    pub EventName: String,
    pub EventTime: f64,
    pub KillerName: Option<String>,
    pub TurretKilled: Option<String>,
    pub Assisters: Option<Vec<String>>,
    pub InhibKilled: Option<String>,
    pub DragonType: Option<String>,
    pub Stolen: Option<String>,
    pub VictimName: Option<String>,
    pub KillStreak: Option<u32>,
    pub Acer: Option<String>,
    pub AcingTeam: Option<String>

}

#[derive(Debug, Deserialize)]
pub struct GameStats {
    pub gameMode: String,
    pub gameTime: f64,
    pub mapName: String,
    pub mapNumber: u32,
    pub mapTerrain: String
}