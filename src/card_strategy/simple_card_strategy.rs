use card_strategy::CardStrategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

#[derive(Debug)]
pub struct SimpleCardStrategy;

impl CardStrategy for SimpleCardStrategy {
    fn pass_cards(&mut self, game_status: &GameStatus) -> Vec<Card> {
        panic!("not implemented")
    }
    fn play_card(game_status: &GameStatus, player_name: &PlayerName) -> Card {
        panic!("not implemented")
    }
}