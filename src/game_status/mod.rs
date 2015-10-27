pub mod dto;

use card::Card;
use deal::Deal;
use player::PlayerName;

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

impl FromStr for GameInstanceState {
    type Err = String;

    fn from_str(string: &str) -> Result<GameInstanceState, String> {
        match string {
            "NotStarted" => Ok(GameInstanceState::NotStarted),
            "Initiated" => Ok(GameInstanceState::Initiated),
            "Open" => Ok(GameInstanceState::Open),
            "Running" => Ok(GameInstanceState::Running),
            "Finished" => Ok(GameInstanceState::Finished),
            "Cancelled" => Ok(GameInstanceState::Cancelled),
            _ => Err(format!("Invalid GameInstanceState: {}", string))
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

impl FromStr for RoundState {
    type Err = String;

    fn from_str(string: &str) -> Result<RoundState, String> {
        match string {
            "NotStarted" => Ok(RoundState::NotStarted),
            "Initiated" => Ok(RoundState::Initiated),
            "Running" => Ok(RoundState::Running),
            "Finished" => Ok(RoundState::Finished),
            "Cancelled" => Ok(RoundState::Cancelled),
            _ => Err(format!("Invalid RoundState: {}", string))
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

impl FromStr for HeartsGameInstanceState {
    type Err = String;

    fn from_str(string: &str) -> Result<HeartsGameInstanceState, String> {
        match string {
            "NotStarted" => Ok(HeartsGameInstanceState::NotStarted),
            "Initiated" => Ok(HeartsGameInstanceState::Initiated),
            "Passing" => Ok(HeartsGameInstanceState::Passing),
            "Dealing" => Ok(HeartsGameInstanceState::Dealing),
            "Finished" => Ok(HeartsGameInstanceState::Finished),
            "Cancelled" => Ok(HeartsGameInstanceState::Cancelled),
            _ => Err(format!("Invalid HeartsGameInstanceState: {}", string))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::dto::*;

    use try_from::TryFrom;

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
        GameStatus::try_from(game_status_dto).unwrap();
    }

    #[test]
    fn read_json2() {
        let mut game_status_file = File::open("gamestatus2.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).unwrap();
        GameStatus::try_from(game_status_dto).unwrap();
    }

}
