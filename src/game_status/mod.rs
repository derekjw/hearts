pub mod dto;

use card::Card;
use deal::Deal;
use player::PlayerName;

use std::collections::BTreeSet;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct GameStatus {
    pub current_game_id: String,
    pub current_game_state: GameInstanceState,
    pub current_round_id: u32,
    pub current_round_state: RoundState,
    pub round_parameters: RoundParameters,
    pub my_game_state: HeartsGameInstanceState,
    pub my_game_players: BTreeSet<PlayerName>,
    pub my_initial_hand: BTreeSet<Card>,
    pub my_final_hand: BTreeSet<Card>,
    pub my_current_hand: BTreeSet<Card>,
    pub my_game_deals: Vec<Deal>,
    pub my_in_progress_deal: Option<Deal>,
    pub is_my_turn: bool,
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

impl<'a> From<&'a str> for GameInstanceState {
    fn from(string: &'a str) -> GameInstanceState {
        match string {
            "NotStarted" => GameInstanceState::NotStarted,
            "Initiated" => GameInstanceState::Initiated,
            "Open" => GameInstanceState::Open,
            "Running" => GameInstanceState::Running,
            "Finished" => GameInstanceState::Finished,
            "Cancelled" => GameInstanceState::Cancelled,
            _ => panic!("Invalid GameInstanceState: {}", string)
        }
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

impl<'a> From<&'a str> for RoundState {
    fn from(string: &'a str) -> RoundState {
        match string {
            "NotStarted" => RoundState::NotStarted,
            "Initiated" => RoundState::Initiated,
            "Running" => RoundState::Running,
            "Finished" => RoundState::Finished,
            "Cancelled" => RoundState::Cancelled,
            _ => panic!("Invalid RoundState: {}", string)
        }
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

#[derive(Debug)]
pub enum HeartsGameInstanceState {
    NotStarted,
    Initiated,
    Passing,
    Dealing,
    Finished,
    Cancelled,
}

impl<'a> From<&'a str> for HeartsGameInstanceState {
    fn from(string: &'a str) -> HeartsGameInstanceState {
        match string {
            "NotStarted" => HeartsGameInstanceState::NotStarted,
            "Initiated" => HeartsGameInstanceState::Initiated,
            "Passing" => HeartsGameInstanceState::Passing,
            "Dealing" => HeartsGameInstanceState::Dealing,
            "Finished" => HeartsGameInstanceState::Finished,
            "Cancelled" => HeartsGameInstanceState::Cancelled,
            _ => panic!("Invalid HeartsGameInstanceState: {}", string)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::dto::*;

    extern crate env_logger;
    extern crate serde;
    extern crate serde_json;

    use std::fs::File;
    use std::io::Read;

    #[test]
    fn read_json() {
        let mut game_status_file = File::open("gamestatus.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).unwrap();
        let game_status = GameStatus::from(game_status_dto);
    }

    #[test]
    fn read_json2() {
        let mut game_status_file = File::open("gamestatus2.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).unwrap();
        let game_status = GameStatus::from(game_status_dto);
    }

}
