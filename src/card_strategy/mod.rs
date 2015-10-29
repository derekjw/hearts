mod simple_card_strategy;
mod defensive_card_strategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

use std::fmt::Debug;

pub use card_strategy::simple_card_strategy::SimpleCardStrategy;
pub use card_strategy::defensive_card_strategy::DefensiveCardStrategy;

pub trait CardStrategy: Debug {
    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card>;
    fn play_card<'a>(&mut self, game_status: &'a GameStatus, player_name: &PlayerName) -> &'a Card;
}