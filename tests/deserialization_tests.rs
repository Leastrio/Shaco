use serde::{Deserialize, Serialize};

use shaco::{
    ingame::IngameClient,
    model::ingame::{AllGameData, GameEvent, GameMode},
};

/// check if all api calls deserialize without errors \
/// DOES NOT CHECK IF THE EVENTS GET DESERIALIZED CORRECTLY
#[tokio::test]
async fn ingame_livegame_api_deserialization() {
    let client = IngameClient::new().unwrap();

    assert!(client.active_game().await);
    assert!(client.active_game_loadingscreen().await);
    assert!(!client.is_spectator_mode().await.unwrap());
    client.all_game_data(None).await.unwrap();
    client.event_data(None).await.unwrap();
    client.game_stats().await.unwrap();

    let players = client.player_list(None).await.unwrap();
    let player_name = players.first().unwrap().summoner_name.to_string();

    client.player_items(&player_name).await.unwrap();
    client.player_main_runes(&player_name).await.unwrap();
    client.player_scores(&player_name).await.unwrap();
    client.player_summoner_spells(&player_name).await.unwrap();

    client.active_player().await.unwrap();
    client.active_player_abilities().await.unwrap();
    client.active_player_name().await.unwrap();
    client.active_player_runes().await.unwrap();
}

/// check if all api calls deserialize without errors \
/// DOES NOT CHECK IF THE EVENTS GET DESERIALIZED CORRECTLY
#[tokio::test]
async fn ingame_spectate_api_deserialization() {
    let client = IngameClient::new().unwrap();

    assert!(client.active_game().await);
    assert!(client.active_game_loadingscreen().await);
    assert!(client.is_spectator_mode().await.unwrap());
    client.all_game_data(None).await.unwrap();
    client.event_data(None).await.unwrap();
    client.game_stats().await.unwrap();

    let players = client.player_list(None).await.unwrap();
    let player_name = players.first().unwrap().summoner_name.to_string();

    client.player_items(&player_name).await.unwrap();
    client.player_main_runes(&player_name).await.unwrap();
    client.player_scores(&player_name).await.unwrap();
    client.player_summoner_spells(&player_name).await.unwrap();
}

#[test]
fn deserialize_data_test() {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct IngameEvents {
        events: Vec<GameEvent>,
    }

    let test_data1 = include_str!("GetLiveclientdataEventdata1.json");
    let test_data2 = include_str!("GetLiveclientdataEventdata2.json");
    let test_data3 = include_str!("aram_allgamedata1.json");
    let test_data4 = include_str!("aram_allgamedata2.json");
    let test_data5 = include_str!("arena_GetLiveclientdataAllgamedata.json");

    assert!(serde_json::from_str::<IngameEvents>(test_data1).is_ok());
    assert!(serde_json::from_str::<IngameEvents>(test_data2).is_ok());
    assert!(serde_json::from_str::<AllGameData>(test_data3).is_ok());
    assert!(serde_json::from_str::<AllGameData>(test_data4).is_ok());
    assert!(serde_json::from_str::<AllGameData>(test_data5).is_ok());

    assert_eq!(
        serde_json::from_str::<IngameEvents>(test_data1)
            .unwrap()
            .events
            .len(),
        127
    );
    assert_eq!(
        serde_json::from_str::<IngameEvents>(test_data2)
            .unwrap()
            .events
            .len(),
        150
    );
}

#[test]
fn deserialize_game_mode() {
    let vec = vec![
        "CLASSIC",
        "ODIN",
        "ARAM",
        "TUTORIAL",
        "URF",
        "DOOMBOTSTEEMO",
        "ONEFORALL",
        "ASCENSION",
        "FIRSTBLOOD",
        "KINGPORO",
        "SIEGE",
        "ASSASSINATE",
        "ARSR",
        "DARKSTAR",
        "STARGUARDIAN",
        "PROJECT",
        "GAMEMODEX",
        "ODYSSEY",
        "NEXUSBLITZ",
        "ULTBOOK",
        "CHERRY",
    ];

    vec.iter().for_each(|game_mode| {
        serde_json::from_str::<GameMode>(&format!("\"{game_mode}\"")).unwrap();
    })
}
