use card_strategy::CardStrategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct DefensiveCardStrategy;

/*
    Play 2 of clubs if in hand.
    If going to win a deal, do so with highest ranking card.
*/
impl DefensiveCardStrategy {
    fn score_card<'a>(card: &'a Card, game_status: &'a GameStatus) -> (i32, i32, i32, i32) {
        let card_penalty_to_me = Self::card_penalty_to_me(card, game_status);
        let card_penalty = 0 - Self::card_penalty(card, game_status);
        let trouble = Self::trouble_score(card, game_status);
        let card_rank = 0 - (u32::from(card.rank) as i32);
        (card_penalty_to_me, card_penalty, trouble, card_rank)
    }

    fn card_penalty(card: &Card, game_status: &GameStatus) -> i32 {
        game_status.round_parameters.card_points.get(card)
            .map(|penalty| *penalty)
            .unwrap_or_default()
    }

    fn card_penalty_to_me(card: &Card, game_status: &GameStatus) -> i32 {
        if Self::will_win_deal(card, game_status) {
            Self::card_penalty(card, game_status)
        } else {
            0
        }
    }

    fn trouble_score(card: &Card, game_status: &GameStatus) -> i32 {
        if Self::will_win_deal(card, game_status) {
            let trouble = u32::from(card.rank) as i32;
            if Self::card_penalty_to_me(card, game_status) < 0 {
                0 - trouble
            } else {
                trouble
            }
        } else {
            0
        }
    }

    fn will_win_deal(card: &Card, game_status: &GameStatus) -> bool {
        game_status.my_in_progress_deal.as_ref().and_then(|deal|
            deal.suit.and_then(|suit|
                deal.deal_cards.iter()
                    .map(|deal_card| deal_card.card)
                    .filter(|card| card.suit == suit)
                    .max()
                    .map(|winning_card| card.suit == suit && card.rank > winning_card.rank))).unwrap_or(true)
    }
}

impl CardStrategy for DefensiveCardStrategy {

    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card> {
        // Need to order cards by potential to win lowest penalty hands
        // make sure high risk cards are passed, including those that will win high risk cards (Queen of Spades)
        let mut initial_hand = game_status.my_initial_hand.iter().collect::<Vec<&Card>>();
        initial_hand.reverse();
        initial_hand.into_iter().take(3).collect()
    }

    fn play_card<'a>(&mut self, game_status: &'a GameStatus, player_name: &PlayerName) -> &'a Card {
        let current_suit = game_status.my_in_progress_deal.as_ref().and_then(|deal| deal.suit);
        let mut valid_cards = game_status.my_current_hand.iter()
            .filter(|card| Some(card.suit) == current_suit)
            .collect::<Vec<&Card>>();

        if valid_cards.is_empty() {
            valid_cards.extend(&game_status.my_current_hand);
        }

        valid_cards.into_iter()
            .map(|card| ((Self::score_card(card, game_status), card), card))
            .collect::<BTreeMap<((i32, i32, i32, i32), &Card), &Card>>()
            .values()
            .next()
            .expect("No valid cards to play!")
    }

}