use card_strategy::CardStrategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

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

    fn remaining_cards(game_status: &GameStatus) -> BTreeSet<Card> {
        Card::all()
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

#[cfg(test)]
mod tests {
    use super::*;
    use player::PlayerName;
    use card::Card;
    use card::Rank::*;
    use card::Suit::*;
    use game_status::GameStatus;
    use game_status::dto::GameStatusDto;
    use card_strategy::CardStrategy;

    use try_from::TryFrom;
    use error::Error;

    extern crate serde;
    extern crate serde_json;

    use std::fs::File;
    use std::io::Read;

    fn open_scenario(name: &str) -> GameStatus {
        let file_name = format!("samples/scenarios/{}.json", name);
        let mut game_status_file = File::open(file_name).unwrap();
        let mut game_status_string = String::new();
        game_status_file.read_to_string(&mut game_status_string).unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string).map_err(Error::from).unwrap();
        GameStatus::try_from(game_status_dto).unwrap()
    }

    fn should_play(name: &str, expected_card: Card) {
        let player_name = PlayerName::new("Derek Williams");
        let game_status = open_scenario(name);
        let card = DefensiveCardStrategy.play_card(&game_status, &player_name).clone();
        assert_eq!(expected_card, card);
    }

    #[test]
    fn normal_1() {
        should_play("normal 1", Jack.of(Diamond));
    }

    #[test]
    fn normal_2() {
        should_play("normal 2", King.of(Club));
    }

    #[test]
    fn normal_3() {
        should_play("normal 3", Six.of(Spade));
    }

    #[test]
    fn should_play_heart_1() { // Opening card should not try to win
        should_play("should play heart 1", Seven.of(Heart));
    }

    #[test]
    fn should_play_heart_2() {
        should_play("should play heart 2", Four.of(Heart));
    }

    #[test]
    fn should_play_high_rank_1() { // Should get rid of high rank
        should_play("should play high rank 1", King.of(Diamond));
    }

    #[test]
    fn should_play_high_rank_2() { // Should get rid of high risk high rank
        should_play("should play high rank 2", Ace.of(Spade));
    }

    #[test]
    fn should_play_high_rank_3() { // Should get rid of high risk high rank
        should_play("should play high rank 3", Ace.of(Spade));
    }

}