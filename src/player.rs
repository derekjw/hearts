use card_strategy::CardStrategy;
use game_status::{
    GameStatus,
    GameInstanceState,
    RoundState,
    HeartsGameInstanceState
};
use game_status::dto::GameStatusDto;
use try_from::TryFrom;
use error::Error;
use error::Result;

use std::io::Read;
use std::collections::BTreeSet;
use std::time::Duration;
use std::thread;

use hyper;
use hyper::Client;
use hyper::client::Response;
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

#[derive(Deserialize, Debug)]
struct GameResponse {
    fault: Option<String>,
    #[serde(rename="hasError")]
    has_error: bool,
    data: String,
}

pub struct Player<A: CardStrategy> {
    player_name: PlayerName,
    password: Password,
    base_url: String,
    card_strategy: A,
    player_activity_tracker: BTreeSet<String>,
    client: Client,
    running: bool,
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
            running: false,
        }
    }

    pub fn play(mut self) {
        self.player_activity_tracker.clear();
        self.check_server_connectivity();
        self.running = true;
        while self.running {
            self.get_game_status()
                .and_then(|game_status| {
                    let state = &game_status.current_game_state;
                    self.update_game_state(state);
                    match state {
                        &GameInstanceState::Open => self.on_game_open(),
                        &GameInstanceState::Finished => self.on_game_finished(),
                        &GameInstanceState::Cancelled => self.on_game_finished(),
                        &GameInstanceState::Running => self.on_game_running(&game_status),
                        _ => Ok(())
                    }
                })
                .unwrap_or_else(|e| error!("Unexpected failure: {}", e));
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

    fn on_game_open(&mut self) -> Result<()> {
        let key_join_status = "JoinGame".to_owned();
        if !self.player_activity_tracker.contains(&key_join_status) {
            if self.join_game() {
                info!("Join successful");
                self.player_activity_tracker.insert(key_join_status);
            }
        };
        Ok(())
    }

    fn on_game_finished(&mut self) -> Result<()> {
        self.running = false;
        Ok(())
    }

    fn on_game_running(&mut self, game_status: &GameStatus) -> Result<()> {
        if game_status.current_round_id > 0 {
            self.update_activity_tracker(format!("Round {} - {:?}", game_status.current_round_id, game_status.current_round_state));

            match game_status.current_round_state {
                RoundState::Running => self.on_round_running(game_status),
                _ => Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn on_round_running(&mut self, game_status: &GameStatus) -> Result<()> {
        self.update_activity_tracker(format!("My game - Round {} {:?}", game_status.current_round_id, game_status.my_game_state));

        match game_status.my_game_state {
            HeartsGameInstanceState::Passing => self.on_passing(game_status),
            HeartsGameInstanceState::Dealing => self.on_dealing(game_status),
            _ => Ok(())
        }
    }

    fn on_passing(&mut self, game_status: &GameStatus) -> Result<()> {
        self.display_my_current_hand(game_status);
        let key_passing = format!("Passing - Round {}", game_status.current_round_id);
        if !self.player_activity_tracker.contains(&key_passing) {
            try!(self.do_passing_activity(game_status));
            self.player_activity_tracker.insert(key_passing);
        }
        Ok(())
    }

    fn on_dealing(&mut self, game_status: &GameStatus) -> Result<()> {
        if game_status.is_my_turn {
            self.display_my_current_hand(game_status);
            let deal_number = game_status.my_in_progress_deal.as_ref().map(|deal| deal.deal_number).unwrap_or_default();
            let key_dealing = format!("Dealing - Round {} Deal {}", game_status.current_round_id, deal_number);
            if !self.player_activity_tracker.contains(&key_dealing) {
                try!(self.do_dealing_activity(game_status));
                self.player_activity_tracker.insert(key_dealing);
            }
        }
        Ok(())
    }

    fn display_my_current_hand(&mut self, game_status: &GameStatus) {
        let key_display_current_hand_cards = game_status.my_current_hand.iter()
            .map(|card| format!("{:?}-{:?}", card.suit, card.rank))
            .collect::<Vec<String>>()
            .join("|");
        let key_display_current_hand = format!("{}-{}", game_status.current_round_id, key_display_current_hand_cards);

        if !self.player_activity_tracker.contains(&key_display_current_hand) {
            let display_current_hand_cards = game_status.my_current_hand.iter()
                .map(|card| format!("{}", card))
                .collect::<Vec<String>>()
                .join(", ");
            info!("My Current Hand : {}", display_current_hand_cards);
            self.player_activity_tracker.insert(key_display_current_hand);
        }


    }

    fn do_passing_activity(&mut self, game_status: &GameStatus) -> Result<()> {
        let number_of_cards_to_be_passed = game_status.round_parameters.number_of_cards_to_be_passed;
        info!("{} cards need to be passed to the right.", number_of_cards_to_be_passed);
        let cards_to_pass = self.card_strategy.pass_cards(game_status);

        let serialized_cards_to_pass = try!(serde_json::to_string(&cards_to_pass));

        self.client
            .post(&format!("{}/passcards", self.base_url))
            .header(self.authorization())
            .header(header::ContentType::json())
            .body(&serialized_cards_to_pass)
            .send()
            .map_err(Error::from)
            .and_then(Self::parse_game_response)
            .map(|data| {
                let passed_cards = cards_to_pass.iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<String>>()
                    .join(", ");

                info!("{} cards passed successfully. Cards are : {}", number_of_cards_to_be_passed, passed_cards);
            })
    }

    fn do_dealing_activity(&mut self, game_status: &GameStatus) -> Result<()> {
        let card_to_deal = self.card_strategy.play_card(game_status, &self.player_name);

        let serialized_card_to_deal = try!(serde_json::to_string(&card_to_deal));

        self.client
            .post(&format!("{}/playcard", self.base_url))
            .header(self.authorization())
            .header(header::ContentType::json())
            .body(&serialized_card_to_deal)
            .send()
            .map_err(Error::from)
            .and_then(Self::parse_game_response)
            .map(|data| info!("{} played Successfully", card_to_deal))
    }

    fn check_server_connectivity(&self) {
        while !self.ping() {
            info!("Trying to connect to server {}", self.base_url);
            thread::sleep(Duration::new(5, 0));
        }
    }

    fn ping(&self) -> bool {
        self.client
            .head(&self.base_url)
            .header(self.authorization())
            .send()
            .is_ok()
    }

    fn get_game_status(&self) -> Result<GameStatus> {
        self.client
            .get(&format!("{}/gamestatus", &self.base_url))
            .header(self.authorization())
            .send()
            .map_err(Error::from)
            .and_then(Self::parse_game_response)
            .and_then(Self::parse_game_status)
    }

    fn join_game(&self) -> bool {
        self.client
            .post(&format!("{}/join", &self.base_url))
            .header(self.authorization())
            .send()
            .is_ok()
    }

    fn parse_game_status(game_response: String) -> Result<GameStatus> {
        serde_json::from_str::<GameStatusDto>(&game_response)
            .map_err(Error::from)
            .and_then(GameStatus::try_from)
    }

    fn parse_game_response(response: Response) -> Result<String> {
        assert_eq!(hyper::Ok, response.status);
        let mut response = response;
        let mut response_body = String::new();
        try!(response.read_to_string(&mut response_body));
        let game_response: GameResponse = try!(serde_json::from_str(&response_body));
        if game_response.has_error {
            Err(Error::game(game_response.fault))
        } else {
            Ok(game_response.data)
        }
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
