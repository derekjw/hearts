use std::collections::BTreeMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct PlayerName {
    value: String
}

impl PlayerName {
    pub fn new<A>(value: A) -> PlayerName
    where A: Into<String> {
        PlayerName { value: value.into() }
    }
}

#[derive(Debug)]
pub struct Player {
    player_name: PlayerName,
    password: String,
    base_url: String,
    card_strategy: String,
    player_activity_tracker: BTreeMap<PlayerName, bool>,
}

impl Player {
    pub fn new<A>(player_name: PlayerName, password: A, hostname: &str) -> Player
    where A: Into<String> {
        let base_url = format!("http://{}/api/participant", hostname);
        Player {
            player_name: player_name,
            password: password.into(),
            base_url: base_url,
            card_strategy: "card_strategy".to_owned(),
            player_activity_tracker: BTreeMap::new(),
        }
    }

    pub fn play(mut self) {
        self.card_strategy = "other_strat".to_owned();
        println!("{:?}", self);
    }
}
