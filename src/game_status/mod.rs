pub mod dto;

use card::Card;
use deal::Deal;

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub struct GameStatus {
    pub current_game_id: String,
    pub current_game_state: GameInstanceState,
    pub current_round_id: u32,
    pub current_round_state: RoundState,
    pub round_parameters: RoundParameters,
    pub game_state: HeartsGameInstanceState,
    pub game_state_description: String,
    pub game_players: Vec<GameParticipant>,
    pub my_initial_hand: BTreeSet<Card>,
    pub cards_passed_by_me: BTreeSet<Card>,
    pub cards_passed_to_me: BTreeSet<Card>,
    pub my_final_hand: BTreeSet<Card>,
    pub my_current_hand: BTreeSet<Card>,
    pub game_deals: Vec<Deal>,
    pub in_progress_deal: Option<Deal>,
    pub is_my_turn: bool,
}

impl GameStatus {
    pub fn unplayed_cards(&self) -> BTreeSet<Card> {
        let mut cards = Card::all();

        for deal in &self.game_deals {
            for deal_card in &deal.deal_cards {
                cards.remove(&deal_card.card);
            }
        }

        if let Some(ref deal) = self.in_progress_deal {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn new<A>(value: A) -> PlayerName
    where A: Into<String> {
        PlayerName(value.into())
    }
}

impl fmt::Display for PlayerName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<PlayerName> for String {
    fn from(player: PlayerName) -> String {
        player.0
    }
}

impl <'a> From<&'a PlayerName> for &'a str {
    fn from(player: &'a PlayerName) -> &'a str {
        &player.0
    }
}

string_enum! {
    GameInstanceState {
        NotStarted,
        Initiated,
        Open,
        Running,
        Finished,
        Cancelled,
    }
}

string_enum! {
    RoundState {
        NotStarted,
        Initiated,
        Running,
        Finished,
        Cancelled,
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
            .cloned()
            .unwrap_or_default()
    }
}

string_enum! {
    HeartsGameInstanceState {
        NotStarted,
        Initiated,
        Passing,
        Dealing,
        Finished,
        Cancelled,
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

    use std::convert::TryFrom;
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
