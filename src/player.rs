use std::collections::BTreeMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct PlayerName (String);

impl PlayerName {
    pub fn new<A>(value: A) -> PlayerName
    where A: Into<String> {
        PlayerName(value.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Password (String);

impl Password {
    pub fn new<A>(value: A) -> Password
    where A: Into<String> {
        Password(value.into())
    }
}

#[derive(Debug)]
pub struct Player {
    player_name: PlayerName,
    password: Password,
    base_url: String,
    card_strategy: String,
    player_activity_tracker: BTreeMap<PlayerName, bool>,
}

impl Player {
    pub fn new(player_name: PlayerName, password: Password, hostname: &str) -> Player {
        let base_url = format!("http://{}/api/participant", hostname);
        Player {
            player_name: player_name,
            password: password,
            base_url: base_url,
            card_strategy: "card_strategy".to_owned(),
            player_activity_tracker: BTreeMap::new(),
        }
    }

    pub fn play(mut self) {
        println!("{:?}", self);
        self.player_activity_tracker.clear();
    }
}
