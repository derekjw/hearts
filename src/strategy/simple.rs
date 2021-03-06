use strategy::CardStrategy;

use card::Card;
use game_status::GameStatus;
use game_status::PlayerName;

#[allow(dead_code)]
#[derive(Debug)]
pub struct SimpleCardStrategy {
    player_name: PlayerName,
}

impl SimpleCardStrategy {
    #[allow(dead_code)]
    pub fn new(player_name: PlayerName) -> SimpleCardStrategy {
        SimpleCardStrategy { player_name: player_name }
    }
}

impl CardStrategy for SimpleCardStrategy {
    fn player_name(&self) -> &PlayerName {
        &self.player_name
    }

    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card> {
        game_status.my_initial_hand.iter().take(3).collect()
    }

    #[allow(unused_variables)]
    fn play_card<'a>(&mut self, game_status: &'a GameStatus) -> &'a Card {
        let current_suit = game_status.in_progress_deal.as_ref().and_then(
            |deal| deal.suit,
        );
        let mut valid_cards: Vec<&'a Card> = game_status
            .my_current_hand
            .iter()
            .filter(|card| Some(card.suit) == current_suit)
            .collect();

        if valid_cards.is_empty() {
            if current_suit.is_some() {
                let mut my_current_hand: Vec<&'a Card> =
                    game_status.my_current_hand.iter().collect();
                my_current_hand.reverse();
                valid_cards.extend(&my_current_hand);
            } else {
                valid_cards.extend(&game_status.my_current_hand);
            }
        }

        valid_cards.first().expect("No valid cards to play!")
    }
}
