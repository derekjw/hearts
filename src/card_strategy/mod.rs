mod simple_card_strategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

pub use card_strategy::simple_card_strategy::SimpleCardStrategy;

pub trait CardStrategy {
    fn pass_cards(&mut self, game_status: &GameStatus) -> Vec<Card>;
    fn play_card(game_status: &GameStatus, player_name: &PlayerName) -> Card;
}