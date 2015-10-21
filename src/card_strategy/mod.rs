mod simple_card_strategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

use std::fmt::Debug;

pub use card_strategy::simple_card_strategy::SimpleCardStrategy;

pub trait CardStrategy: Debug {
    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card>;
    fn play_card<'a>(game_status: &'a GameStatus, player_name: &PlayerName) -> &'a Card;
}