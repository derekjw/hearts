
pub mod card;

use card::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[allow(dead_code)]
fn main() {
    let team_name = PlayerName::new("FlyingBirds");
    let password = "mypassword";

    println!("Start Game");

    // Settings.init();
    let player = Player::new(team_name, password, "localhost");
    player.play();
}

#[derive(Debug)]
struct PlayerName {
    value: String
}

impl PlayerName {
    pub fn new(value: &str) -> PlayerName {
        PlayerName { value: value.to_owned() }
    }
}

struct Settings {
    username: String,
    password: String,
    hostname: String,
}

#[derive(Debug)]
struct Player {
    team_name: PlayerName,
    password: String,
    base_url: String,
    card_strategy: String,
    player_activity_tracker: String,
}

impl Player {
    pub fn new(team_name: PlayerName, password: &str, hostname: &str) -> Player {
        let base_url = format!("http://{}/api/participant", hostname);
        Player {
            team_name: team_name,
            password: password.to_owned(),
            base_url: base_url,
            card_strategy: "card_strategy".to_owned(),
            player_activity_tracker: "player_activity_tracker".to_owned(),
        }
    }

    pub fn play(mut self) {
        self.card_strategy = "other_strat".to_owned();
        println!("{:?}", self);
    }
}

struct GameStatus {
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

struct Deal {
    deal_number: u32,
    initiator: PlayerName,
    suit_type: Suit,
    deal_cards: BTreeSet<DealCard>,
    deal_winner: PlayerName,
}

struct DealCard {
    team_name: PlayerName,
    card: Card,
}