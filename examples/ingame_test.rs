use shaco::ingame::IngameClient;

#[tokio::main]
async fn main() {
    let client = IngameClient::new();

    let mut loading_screen = false;
    let mut ingame = false;
    let mut spectator = false;
    loop {
        let mut print = false;

        let new_loading_screen = client.active_game_loadingscreen().await;
        if new_loading_screen != loading_screen {
            loading_screen = new_loading_screen;
            print = true;
        }

        let new_ingame = client.active_game().await;
        if new_ingame != ingame {
            ingame = new_ingame;
            print = true;

            if ingame {
                let time = client
                    .all_game_data(None)
                    .await
                    .unwrap()
                    .game_data
                    .game_time;
                println!("time: {time}");
            }
        }

        let new_spectator = client.is_spectator_mode().await.is_ok_and(|b| b);
        if new_spectator != spectator {
            spectator = new_spectator;
            print = true;
        }

        if print {
            println!("loading screen: {loading_screen}");
            println!("ingame: {ingame}");
            println!("spectator: {spectator}");
        }
    }
}
