use card_strategy::CardStrategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

#[derive(Debug)]
pub struct SimpleCardStrategy;

impl CardStrategy for SimpleCardStrategy {

    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card> {
        game_status.my_initial_hand.iter().take(3).collect()
    }

    fn play_card<'a>(&mut self, game_status: &'a GameStatus, player_name: &PlayerName) -> &'a Card {
        let mut valid_cards: Vec<&'a Card> = game_status.my_current_hand.iter().filter(|card| card.suit == game_status.my_in_progress_deal.suit).collect();

        if valid_cards.is_empty() {
            valid_cards.extend(&game_status.my_current_hand);
        }

        valid_cards.first().expect("No valid cards to play!")
    }

}