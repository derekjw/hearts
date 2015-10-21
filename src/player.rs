use card_strategy::CardStrategy;

use std::io::Read;
use std::collections::BTreeMap;
use std::time::Duration;
use std::thread;

use hyper::Client;
use hyper::header::Connection;

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

pub struct Player<A: CardStrategy> {
    player_name: PlayerName,
    password: Password,
    base_url: String,
    card_strategy: A,
    player_activity_tracker: BTreeMap<PlayerName, bool>,
    client: Client,
}

impl<A: CardStrategy> Player<A> {
    pub fn new(player_name: PlayerName, password: Password, hostname: &str, card_strategy: A) -> Player<A> {
        let base_url = format!("http://{}/api/participant", hostname);
        Player {
            player_name: player_name,
            password: password,
            base_url: base_url,
            card_strategy: card_strategy,
            player_activity_tracker: BTreeMap::new(),
            client: Client::new(),
        }
    }

    pub fn play(mut self) {
        self.player_activity_tracker.clear();
        self.check_server_connectivity();
    }

    fn check_server_connectivity(&self) {
        while !self.ping() {
            info!("Trying to connect to server {}", self.base_url);
            thread::sleep(Duration::new(5, 0));
        }
    }

    fn ping(&self) -> bool {
        self.client.head(&self.base_url).send().is_ok()
    }
}
