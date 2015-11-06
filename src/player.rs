use card_strategy::CardStrategy;
use game_status::{
    GameStatus,
    GameInstanceState,
    RoundState,
    HeartsGameInstanceState
};
use game_status::dto::GameStatusDto;
use card::dto::CardDto;
use try_from::TryFrom;
use error::Error;
use error::Result;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::BTreeSet;
use std::time::Duration;
use std::thread;

use hyper;
use hyper::Client;
use hyper::client::Response;
use hyper::header;

use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn new<A>(value: A) -> PlayerName
    where A: Into<String> {
        PlayerName(value.into())
    }
}

impl From<PlayerName> for String {
    fn from(player: PlayerName) -> String {
        let PlayerName(string) = player;
        string
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Password(String);

impl Password {
    pub fn new<A>(value: A) -> Password
    where A: Into<String> {
        Password(value.into())
    }
}

impl From<Password> for String {
    fn from(password: Password) -> String {
        let Password(string) = password;
        string
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
    repeat: bool,
    current_game_id: Option<String>,
}

impl<A: CardStrategy> Player<A> {
    pub fn new(player_name: PlayerName, password: Password, hostname: &str, card_strategy: A, repeat: bool) -> Player<A> {
        let base_url = format!("http://{}/api/participant", hostname);
        Player {
            player_name: player_name,
            password: password,
            base_url: base_url,
            card_strategy: card_strategy,
            player_activity_tracker: BTreeSet::new(),
            client: Client::new(),
            running: false,
            repeat: repeat,
            current_game_id: None,
        }
    }

    pub fn play(mut self) {
        self.running = true;
        self.check_server_connectivity();
        while self.running {
            self.get_game_status()
                .and_then(|game_status| {
                    self.set_current_game_id(&game_status.current_game_id);
                    let state = &game_status.current_game_state;
                    self.update_game_state(state);
                    match *state {
                        GameInstanceState::Open => self.on_game_open(),
                        GameInstanceState::Finished => self.on_game_finished(),
                        GameInstanceState::Cancelled => self.on_game_finished(),
                        GameInstanceState::Running => self.on_game_running(&game_status),
                        _ => Ok(())
                    }
                })
                .unwrap_or_else(|e| error!("Unexpected failure: {}", e));
            thread::sleep(Duration::new(1, 0));
        }
    }

    fn set_current_game_id(&mut self, game_id: &str) {
        if self.current_game_id.as_ref().map(|current_game_id| current_game_id != game_id).unwrap_or(true) {
            self.player_activity_tracker.clear();
            self.current_game_id = Some(game_id.to_owned())
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
        if !self.repeat {
            self.running = false;
        }
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
            try!(self.log_game_status(game_status, 0));
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
                try!(self.log_game_status(game_status, deal_number));
                try!(self.do_dealing_activity(game_status));
                self.player_activity_tracker.insert(key_dealing);
            }
        }
        Ok(())
    }

    fn log_game_status(&self, game_status: &GameStatus, deal_number: u32) -> Result<()> {
        let ref game_id = game_status.current_game_id;
        let round_id = game_status.current_round_id;
        let dir_name = format!("game_log/{}", game_id);
        try!(fs::DirBuilder::new().recursive(true).create(&dir_name));
        let file_name = format!("{}/{:02}-{:02}.json", dir_name, round_id, deal_number);
        let mut file = try!(File::create(file_name));
        let dto = GameStatusDto::from(game_status);
        let string = try!(serde_json::to_string_pretty(&dto));
        try!(file.write(&string.into_bytes()));
        try!(file.flush());
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
        let cards_to_pass_dto = cards_to_pass.iter().map(|&card| card.into()).collect::<Vec<CardDto>>();

        let serialized_cards_to_pass = try!(serde_json::to_string(&cards_to_pass_dto));

        self.client
            .post(&format!("{}/passcards", self.base_url))
            .header(self.authorization())
            .header(header::ContentType::json())
            .body(&serialized_cards_to_pass)
            .send()
            .map_err(Error::from)
            .and_then(Self::parse_game_response)
            .map(|_| {
                let passed_cards = cards_to_pass.iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<String>>()
                    .join(", ");

                info!("{} cards passed successfully. Cards are : {}", number_of_cards_to_be_passed, passed_cards);
            })
    }

    fn do_dealing_activity(&mut self, game_status: &GameStatus) -> Result<()> {
        let card_to_deal = self.card_strategy.play_card(game_status, &self.player_name);
        let card_to_deal_dto: CardDto = card_to_deal.into();

        let serialized_card_to_deal = try!(serde_json::to_string(&card_to_deal_dto));

        self.client
            .post(&format!("{}/playcard", self.base_url))
            .header(self.authorization())
            .header(header::ContentType::json())
            .body(&serialized_card_to_deal)
            .send()
            .map_err(Error::from)
            .and_then(Self::parse_game_response)
            .map(|_| info!("{} played Successfully", card_to_deal))
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
                username: self.player_name.clone().into(),
                password: Some(self.password.clone().into())
            }
        )
    }

}
