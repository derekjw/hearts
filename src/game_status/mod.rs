pub mod dto;

use card::Card;
use deal::Deal;
use player::PlayerName;

use error::Error;

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct GameStatus {
    pub current_game_id: String,
    pub current_game_state: GameInstanceState,
    pub current_round_id: u32,
    pub current_round_state: RoundState,
    pub round_parameters: RoundParameters,
    pub my_game_state: HeartsGameInstanceState,
    pub my_game_state_description: String,
    pub my_game_players: Vec<GameParticipant>,
    pub my_initial_hand: BTreeSet<Card>,
    pub cards_passed_by_me: BTreeSet<Card>,
    pub cards_passed_to_me: BTreeSet<Card>,
    pub my_final_hand: BTreeSet<Card>,
    pub my_current_hand: BTreeSet<Card>,
    pub my_game_deals: Vec<Deal>,
    pub my_in_progress_deal: Option<Deal>,
    pub is_my_turn: bool,
}

impl GameStatus {
    pub fn unplayed_cards(&self) -> BTreeSet<Card> {
        let mut cards = Card::all();

        for deal in &self.my_game_deals {
            for deal_card in &deal.deal_cards {
                cards.remove(&deal_card.card);
            }
        }

        if let &Some(ref deal) = &self.my_in_progress_deal {
            for deal_card in &deal.deal_cards {
                cards.remove(&deal_card.card);
            }
        }

        for card in &self.my_current_hand {
            cards.remove(&card);
        }

        cards
    }

}

#[derive(Debug)]
pub enum GameInstanceState {
    NotStarted,
    Initiated,
    Open,
    Running,
    Finished,
    Cancelled,
}

impl FromStr for GameInstanceState {
    type Err = Error;

    fn from_str(string: &str) -> Result<GameInstanceState, Error> {
        match string {
            "NotStarted" => Ok(GameInstanceState::NotStarted),
            "Initiated" => Ok(GameInstanceState::Initiated),
            "Open" => Ok(GameInstanceState::Open),
            "Running" => Ok(GameInstanceState::Running),
            "Finished" => Ok(GameInstanceState::Finished),
            "Cancelled" => Ok(GameInstanceState::Cancelled),
            _ => Err(Error::parsing("GameInstanceState", string))
        }
    }
}

impl <'a> From<&'a GameInstanceState> for String {
    fn from(state: &'a GameInstanceState) -> String {
        format!("{:?}", state)
    }
}

#[derive(Debug)]
pub enum RoundState {
    NotStarted,
    Initiated,
    Running,
    Finished,
    Cancelled,
}

impl FromStr for RoundState {
    type Err = Error;

    fn from_str(string: &str) -> Result<RoundState, Error> {
        match string {
            "NotStarted" => Ok(RoundState::NotStarted),
            "Initiated" => Ok(RoundState::Initiated),
            "Running" => Ok(RoundState::Running),
            "Finished" => Ok(RoundState::Finished),
            "Cancelled" => Ok(RoundState::Cancelled),
            _ => Err(Error::parsing("RoundState", string))
        }
    }
}

impl <'a> From<&'a RoundState> for String {
    fn from(state: &'a RoundState) -> String {
        format!("{:?}", state)
    }
}

#[derive(Debug)]
pub struct RoundParameters {
    pub round_id: u32,
    pub initiation_phase_in_seconds: u32,
    pub passing_phase_in_seconds: u32,
    pub dealing_phase_in_seconds: u32,
    pub finishing_phase_in_seconds: u32,
    pub number_of_cards_to_be_passed: u32,
    pub card_points: BTreeMap<Card, i32>
}

impl RoundParameters {
    pub fn points(&self, card: &Card) -> i32 {
        self.card_points.get(card)
            .map(|penalty| *penalty)
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub enum HeartsGameInstanceState {
    NotStarted,
    Initiated,
    Passing,
    Dealing,
    Finished,
    Cancelled,
}

impl FromStr for HeartsGameInstanceState {
    type Err = Error;

    fn from_str(string: &str) -> Result<HeartsGameInstanceState, Error> {
        match string {
            "NotStarted" => Ok(HeartsGameInstanceState::NotStarted),
            "Initiated" => Ok(HeartsGameInstanceState::Initiated),
            "Passing" => Ok(HeartsGameInstanceState::Passing),
            "Dealing" => Ok(HeartsGameInstanceState::Dealing),
            "Finished" => Ok(HeartsGameInstanceState::Finished),
            "Cancelled" => Ok(HeartsGameInstanceState::Cancelled),
            _ => Err(Error::parsing("HeartsGameInstanceState", string))
        }
    }
}

impl <'a> From<&'a HeartsGameInstanceState> for String {
    fn from(state: &'a HeartsGameInstanceState) -> String {
        format!("{:?}", state)
    }
}

#[derive(Debug)]
pub struct GameParticipant {
    pub team_name: PlayerName,
    pub left_participant: PlayerName,
    pub number_of_cards_in_hand: u32,
    pub has_turn: bool,
    pub current_score: i32,
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::dto::*;

    use try_from::TryFrom;
    use error::Error;

    extern crate env_logger;
    extern crate serde;
    extern crate serde_json;

    use std::fs::File;
    use std::io::Read;

    #[test]
    fn read_json() {
        let mut game_status_file = File::open("samples/gamestatus.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).map_err(Error::from).unwrap();
        GameStatus::try_from(game_status_dto).unwrap();
    }

    #[test]
    fn read_json2() {
        let mut game_status_file = File::open("samples/gamestatus2.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).map_err(Error::from).unwrap();
        GameStatus::try_from(game_status_dto).unwrap();
    }

    #[test]
    fn read_json3() {
        let mut game_status_file = File::open("samples/gamestatus3.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).map_err(Error::from).unwrap();
        GameStatus::try_from(game_status_dto).unwrap();
    }

    #[test]
    fn read_json4() {
        let mut game_status_file = File::open("samples/gamestatus4.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).map_err(Error::from).unwrap();
        GameStatus::try_from(game_status_dto).unwrap();
    }

    #[test]
    fn read_json5() {
        let mut game_status_file = File::open("samples/gamestatus5.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).map_err(Error::from).unwrap();
        GameStatus::try_from(game_status_dto).unwrap();
    }

}
