use card_strategy::CardStrategy;

use card::Card;
use game_status::GameStatus;
use player::PlayerName;

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MyCardStrategy;

impl MyCardStrategy {
    // Lowest penalty for me using highest penalty card.
    // Lowest penalty for me using highest ranked card.
    // Lowest potential penalty for me using highest ranked card.
    // filter valid cards
    // order by: penalty to me ascending, penalty descending, rank descending
    fn score_card<'a>(card: &'a Card, game_status: &'a GameStatus, player_name: &PlayerName) -> (i32, i32, i32, i32) {
        let card_penalty_to_me = MyCardStrategy::card_penalty_to_me(card, game_status, player_name);
        let card_penalty = 0 - MyCardStrategy::card_penalty(card, game_status);
        let trouble = MyCardStrategy::trouble_score(card, game_status);
        let card_rank = 0 - (u32::from(card.rank) as i32);
        (card_penalty_to_me, card_penalty, trouble, card_rank)
    }

    fn card_penalty(card: &Card, game_status: &GameStatus) -> i32 {
        game_status.round_parameters.card_points.get(card).map(|penalty| penalty.clone()).unwrap_or_default()
    }

    fn card_penalty_to_me(card: &Card, game_status: &GameStatus, player_name: &PlayerName) -> i32 {
        if MyCardStrategy::will_win_deal(card, game_status) {
            MyCardStrategy::card_penalty(card, game_status)
        } else {
            0
        }
    }

    fn trouble_score(card: &Card, game_status: &GameStatus) -> i32 {
        if MyCardStrategy::will_win_deal(card, game_status) {
            let rank_number: u32 = card.rank.into();
            rank_number as i32
        } else {
            0
        }
    }

    fn will_win_deal(card: &Card, game_status: &GameStatus) -> bool {
        match game_status.my_in_progress_deal {
            Some(ref deal) => {
                match deal.suit {
                    Some(current_suit) => {
                        let mut option_winning_card: Option<Card> = None;

                        for deal_card in deal.deal_cards.iter() {
                            if deal_card.card.suit == current_suit {
                                match option_winning_card {
                                    None => option_winning_card = Some(deal_card.card.clone()),
                                    Some(winning_card) => {
                                        if deal_card.card.rank > winning_card.rank {
                                            option_winning_card = Some(deal_card.card.clone())
                                        }
                                    }
                                }
                            }
                        }

                        let will_win_rank = match option_winning_card {
                            Some(winning_card) => card.rank > winning_card.rank,
                            None => true
                        };

                        card.suit == current_suit && will_win_rank
                    }
                    None => true
                }
            }
            None => true
        }
    }
}

impl CardStrategy for MyCardStrategy {

    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card> {
        game_status.my_initial_hand.iter().take(3).collect()
    }

    fn play_card<'a>(&mut self, game_status: &'a GameStatus, player_name: &PlayerName) -> &'a Card {
        let current_suit = game_status.my_in_progress_deal.as_ref().and_then(|deal| deal.suit);
        let mut valid_cards: Vec<&'a Card> = game_status.my_current_hand.iter().filter(|card| Some(card.suit) == current_suit).collect();

        if valid_cards.is_empty() {
            valid_cards.extend(&game_status.my_current_hand);
        }

        let scored_cards: BTreeMap<((i32, i32, i32, i32), &Card), &Card> = valid_cards.into_iter().map(|card| {
            ((MyCardStrategy::score_card(card, game_status, player_name), card), card)
        }).collect();

        scored_cards.values().next().expect("No valid cards to play!")
    }

}