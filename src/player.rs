use card_strategy::CardStrategy;
use game_status::{
    GameStatus,
    GameInstanceState,
    RoundState,
    HeartsGameInstanceState
};
use game_status::dto::GameStatusDto;
use std::io::Read;
use std::collections::BTreeSet;
use std::time::Duration;
use std::thread;

use hyper;
use hyper::Client;
use hyper::header;

use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct PlayerName (String);

impl PlayerName {
    pub fn new<A>(value: A) -> PlayerName
    where A: Into<String> {
        PlayerName(value.into())
    }

    fn clone_string(&self) -> String {
        let &PlayerName(ref string) = self;
        string.clone()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Password (String);

impl Password {
    pub fn new<A>(value: A) -> Password
    where A: Into<String> {
        Password(value.into())
    }

    fn clone_string(&self) -> String {
        let &Password(ref string) = self;
        string.clone()
    }
}

pub struct Player<A: CardStrategy> {
    player_name: PlayerName,
    password: Password,
    base_url: String,
    card_strategy: A,
    player_activity_tracker: BTreeSet<String>,
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
            player_activity_tracker: BTreeSet::new(),
            client: Client::new(),
        }
    }

    pub fn play(mut self) {
        self.player_activity_tracker.clear();
        self.check_server_connectivity();
        let mut running = true;
        while running {
            let game_status = self.get_game_status();
            let state = &game_status.current_game_state;
            self.update_game_state(state);
            match state {
                &GameInstanceState::Open => self.on_game_open(),
                &GameInstanceState::Finished => running = false,
                &GameInstanceState::Cancelled => running = false,
                &GameInstanceState::Running => self.on_game_running(&game_status),
                _ => ()
            }
            thread::sleep(Duration::new(1, 0));
        }
    }

    fn update_game_state(&mut self, state: &GameInstanceState) {
        self.update_activity_tracker(format!("Game State - {:?}", state))
    }

    fn update_activity_tracker(&mut self, key: String) {
        if !self.player_activity_tracker.contains(&key) {
            info!("{}", key);
            self.player_activity_tracker.insert(key);
        }
    }

    fn on_game_open(&mut self) {
        let key_join_status = "JoinGame".to_owned();
        if !self.player_activity_tracker.contains(&key_join_status) {
            if self.join_game() {
                info!("Join successful");
                self.player_activity_tracker.insert(key_join_status);
            }
        }
    }

    fn on_game_running(&mut self, game_status: &GameStatus) {
        if game_status.current_round_id > 0 {
            self.update_activity_tracker(format!("Round {} - {:?}", game_status.current_round_id, game_status.current_round_state));

            match game_status.current_round_state {
                RoundState::Running => self.on_round_running(game_status),
                _ => ()
            }
        }
    }

    fn on_round_running(&mut self, game_status: &GameStatus) {
        self.update_activity_tracker(format!("My game - Round {} {:?}", game_status.current_round_id, game_status.my_game_state));

        match game_status.my_game_state {
            HeartsGameInstanceState::Passing => self.on_passing(game_status),
            HeartsGameInstanceState::Dealing => self.on_dealing(game_status),
            _ => ()
        }
    }

    fn on_passing(&mut self, game_status: &GameStatus) {
        self.display_my_current_hand(game_status);
        let key_passing = format!("Passing - Round {}", game_status.current_round_id);
        if !self.player_activity_tracker.contains(&key_passing) {
            self.do_passing_activity(game_status);
            self.player_activity_tracker.insert(key_passing);
        }
    }

    fn on_dealing(&mut self, game_status: &GameStatus) {
        self.display_my_current_hand(game_status);
        let key_dealing = format!("Dealing - Round {} Deal {}", game_status.current_round_id, game_status.my_in_progress_deal.clone().unwrap().deal_number);
        if !self.player_activity_tracker.contains(&key_dealing) {
            self.do_dealing_activity(game_status);
            self.player_activity_tracker.insert(key_dealing);
        }
    }

    fn display_my_current_hand(&mut self, game_status: &GameStatus) {
        let key_display_current_hand_cards = game_status.my_current_hand.iter()
            .map(|card| format!("{:?}-{:?}", card.suit, card.rank))
            .collect::<Vec<String>>()
            .join("|");
        let key_display_current_hand = format!("{}-{}", game_status.current_round_id, key_display_current_hand_cards);

        if !self.player_activity_tracker.contains(&key_display_current_hand) {
            info!("My Current Hand : ");
            for card in &game_status.my_current_hand {
                info!("{:?} {:?}", card.suit, card.rank);
            }
            self.player_activity_tracker.insert(key_display_current_hand);
        }


    }

    fn do_passing_activity(&mut self, game_status: &GameStatus) {
        let number_of_cards_to_be_passed = game_status.round_parameters.number_of_cards_to_be_passed;
        info!("{} cards need to be passed to the right.", number_of_cards_to_be_passed);
        let cards_to_pass = self.card_strategy.pass_cards(game_status);

        let serialized_cards_to_pass = serde_json::to_string(&cards_to_pass).unwrap();
        match self.client.post(&format!("{}/passcards", self.base_url)).body(&serialized_cards_to_pass).send() {
            Err(e) => error!("Problem while passing: {}", e),
            Ok(response) => {
                info!("{} cards passed successfully. Cards are :", number_of_cards_to_be_passed);
                for card in &cards_to_pass {
                    info!("{:?} {:?}", card.suit, card.rank);
                }
            }
        }
    }

    fn do_dealing_activity(&mut self, game_status: &GameStatus) {
        let card_to_deal = self.card_strategy.play_card(game_status, &self.player_name);

        let serialized_card_to_deal = serde_json::to_string(&card_to_deal).unwrap();
        match self.client.post(&format!("{}/playcard", self.base_url)).body(&serialized_card_to_deal).send() {
            Err(e) => error!("Problem while playing card: {}", e),
            Ok(response) => {
                info!("Card {:?} {:?} played Successfully", card_to_deal.suit, card_to_deal.rank);
            }
        }
    }

    fn check_server_connectivity(&self) {
        while !self.ping() {
            info!("Trying to connect to server {}", self.base_url);
            thread::sleep(Duration::new(5, 0));
        }
    }

    fn ping(&self) -> bool {
        self.client.head(&self.base_url).header(self.authorization()).send().is_ok()
    }

    fn get_game_status(&self) -> GameStatus {
        match self.client.get(&format!("{}/gamestatus", &self.base_url)).header(self.authorization()).send() {
            Err(e) => panic!("OH NOES {:?}", e),
            Ok(mut response) => {
                assert_eq!(hyper::Ok, response.status);
                let mut response_body = String::new();
                response.read_to_string(&mut response_body).unwrap();
                let game_response: GameResponse = serde_json::from_str(&response_body).unwrap();
                // info!("{}", game_response.data);
                let game_status_dto: GameStatusDto = serde_json::from_str(&game_response.data).unwrap();
                GameStatus::from(game_status_dto)
            }
        }
    }

    fn join_game(&self) -> bool {
        self.client.post(&format!("{}/join", &self.base_url)).header(self.authorization()).send().is_ok()
    }

    fn authorization(&self) -> header::Authorization<header::Basic> {
        header::Authorization(
            header::Basic {
                username: self.player_name.clone_string(),
                password: Some(self.password.clone_string())
            }
        )
    }
}


#[derive(Deserialize, Debug)]
struct GameResponse {
    fault: Option<String>,
    #[serde(rename="hasError")]
    has_error: bool,
    data: String,
}