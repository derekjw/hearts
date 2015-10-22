mod dto;

use card::Card;
use deal::Deal;
use player::PlayerName;

use std::collections::BTreeSet;
use std::collections::BTreeMap;

pub struct GameStatus {
    pub current_game_id: String,
    pub current_game_state: GameInstanceState,
    pub current_round_id: u32,
    pub current_round_state: RoundState,
    pub round_parameters: RoundParameters,
    pub my_game_state: HeartsGameInstanceState,
    pub my_game_players: BTreeSet<PlayerName>,
    pub my_left_player: PlayerName,
    pub my_initial_hand: BTreeSet<Card>,
    pub my_final_hand: BTreeSet<Card>,
    pub my_current_hand: BTreeSet<Card>,
    pub my_game_deals: Vec<Deal>,
    pub my_in_progress_deal: Deal,
    pub is_my_turn: bool,
}

pub enum GameInstanceState {
    NotStarted,
    Initiated,
    Open,
    Running,
    Finished,
    Cancelled,
}

pub enum RoundState {
    NotStarted,
    Initiated,
    Running,
    Finished,
    Cancelled,
}

pub struct RoundParameters {
    round_id: u32,
    initiation_phase_in_seconds: u32,
    passing_phase_in_seconds: u32,
    dealing_phase_in_seconds: u32,
    finishing_phase_in_seconds: u32,
    number_of_cards_to_be_passed: u32,
    card_points: BTreeMap<Card, i32>
}

pub enum HeartsGameInstanceState {
    NotStarted,
    Initiated,
    Passing,
    Dealing,
    Finished,
    Cancelled,
}

#[cfg(test)]
mod tests {
    // use super::*;
    use super::dto::*;

    extern crate env_logger;
    extern crate serde;
    extern crate serde_json;

    use std::fs::File;
    use std::io::Read;

    #[test]
    fn read_json() {
        env_logger::init().unwrap();
        let mut game_status_file = File::open("gamestatus.json").unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status: GameStatusDto = serde_json::from_str(&game_status_string).unwrap();
    }

}
