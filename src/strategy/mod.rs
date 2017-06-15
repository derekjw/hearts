mod simple;
mod defensive;

use card::Card;
use game_status::GameStatus;
use game_status::PlayerName;

use std::fmt::Debug;

pub use strategy::simple::SimpleCardStrategy;
pub use strategy::defensive::DefensiveCardStrategy;

pub trait CardStrategy: Debug {
    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card>;
    fn play_card<'a>(&mut self, game_status: &'a GameStatus) -> &'a Card;
    fn player_name(&self) -> &PlayerName;
}
