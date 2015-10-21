use card::Card;
use deal::Deal;
use player::PlayerName;

use std::collections::BTreeSet;
use std::collections::BTreeMap;

pub struct GameStatus {
    current_game_id: String,
    current_game_state: GameInstanceState,
    current_round_id: u32,
    current_round_state: RoundState,
    round_parameters: RoundParameters,
    my_game_state: HeartsGameInstanceState,
    my_game_players: BTreeSet<PlayerName>,
    my_left_player: PlayerName,
    my_initial_hand: BTreeSet<Card>,
    my_final_hand: BTreeSet<Card>,
    my_current_hand: BTreeSet<Card>,
    my_game_deals: Vec<Deal>,
    my_in_progress_deal: Deal,
    is_my_turn: bool,
}

enum GameInstanceState {
    NotStarted,
    Initiated,
    Open,
    Running,
    Finished,
    Cancelled,
}

enum RoundState {
    NotStarted,
    Initiated,
    Running,
    Finished,
    Cancelled,
}

struct RoundParameters {
    round_id: u32,
    initiation_phase_in_seconds: u32,
    passing_phase_in_seconds: u32,
    dealing_phase_in_seconds: u32,
    finishing_phase_in_seconds: u32,
    numberOfCardsToBePassed: u32,
    card_points: BTreeMap<Card, i32>
}

enum HeartsGameInstanceState {
    NotStarted,
    Initiated,
    Passing,
    Dealing,
    Finished,
    Cancelled,
}
