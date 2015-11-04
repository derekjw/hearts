use card_strategy::CardStrategy;

use card::Card;
use card::Suit;
use card::Rank;
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
    fn score_card<'a>(card: &'a Card, game_status: &'a GameStatus) -> (i32, i32, i32, i32, i32, i32, i32) {
        let potential_points = Self::potential_points(card, game_status);
        let definite_points = if Self::will_win_deal(card, game_status) {
            potential_points
        } else {
            0
        };
        let later_potential_points = 0 - Self::later_potential_points(card, game_status);
        let card_penalty_to_me = Self::card_penalty_to_me(card, game_status);
        let card_penalty = 0 - Self::card_penalty(card, game_status);
        let trouble = Self::trouble_score(card, game_status);
        let card_rank = 0 - (u32::from(card.rank) as i32);
        (definite_points, potential_points, later_potential_points, card_penalty_to_me, card_penalty, trouble, card_rank)
    }

    fn card_penalty(card: &Card, game_status: &GameStatus) -> i32 {
        game_status.round_parameters.card_points.get(card)
            .map(|penalty| *penalty)
            .unwrap_or_default()
    }

    fn card_penalty_to_me(card: &Card, game_status: &GameStatus) -> i32 {
        if Self::can_win_deal(card, game_status) {
            Self::card_penalty(card, game_status)
        } else {
            0
        }
    }

    fn trouble_score(card: &Card, game_status: &GameStatus) -> i32 {
        if Self::can_win_deal(card, game_status) {
            let trouble = u32::from(card.rank) as i32;
            if Self::card_penalty_to_me(card, game_status) < 0 {
                0 - trouble
            } else {
                if Self::will_win_deal(card, game_status) {
                    u32::from(Rank::Ace) as i32
                } else {
                    trouble
                }
            }
        } else {
            0
        }
    }

    fn can_win_deal(card: &Card, game_status: &GameStatus) -> bool {
        game_status.my_in_progress_deal.as_ref().and_then(|deal|
            deal.suit.and_then(|suit|
                deal.deal_cards.iter()
                    .map(|deal_card| deal_card.card)
                    .filter(|card| card.suit == suit)
                    .max()
                    .map(|winning_card| card.suit == suit && card.rank > winning_card.rank))).unwrap_or(true)
    }

    fn will_win_deal(card: &Card, game_status: &GameStatus) -> bool {
        Self::can_win_deal(card, game_status) && (Self::plays_left(game_status).len() == 0 ||
            Self::deal_suit(game_status)
                .and_then(|suit| Self::remaining_cards(game_status).iter()
                    .filter(|card| card.suit == *suit)
                    .max()
                    .map(|winning_card| card.rank > winning_card.rank)).unwrap_or(true))
    }

    fn deal_suit(game_status: &GameStatus) -> Option<&Suit> {
        game_status.my_in_progress_deal.as_ref().and_then(|deal| deal.suit.as_ref())
    }

    fn plays_left(game_status: &GameStatus) -> BTreeSet<&PlayerName> {
        let mut players: BTreeSet<&PlayerName> = game_status.my_game_players.iter()
            .filter(|player| !player.has_turn)
            .map(|player| &player.team_name)
            .collect();

        if let &Some(ref deal) = &game_status.my_in_progress_deal {
            for deal_card in &deal.deal_cards {
                players.remove(&deal_card.player_name);
            }
        }

        players
    }

    fn remaining_cards(game_status: &GameStatus) -> BTreeSet<Card> {
        let mut cards = Card::all();

        for deal in &game_status.my_game_deals {
            for deal_card in &deal.deal_cards {
                cards.remove(&deal_card.card);
            }
        }

        if let &Some(ref deal) = &game_status.my_in_progress_deal {
            for deal_card in &deal.deal_cards {
                cards.remove(&deal_card.card);
            }
        }

        for card in &game_status.my_current_hand {
            cards.remove(&card);
        }

        cards
    }

    fn dealt_cards(game_status: &GameStatus) -> BTreeSet<Card> {
        game_status.my_in_progress_deal.as_ref()
            .map(|deal| deal.deal_cards.iter().map(|deal_card| deal_card.card).collect())
            .unwrap_or_default()
    }

    // FIXME: needs to take into account card that will be played!
    fn potential_points(card: &Card, game_status: &GameStatus) -> i32 {
        if Self::can_win_deal(card, game_status) {
            let cards = Self::remaining_cards(game_status);

            let dealt_points: i32 = Self::dealt_cards(game_status).iter()
                .map(|other| Self::card_penalty(other, game_status))
                .sum();

            let suit_points: i32 = cards.iter()
                .filter(|other| other.suit == card.suit)
                .filter(|other| other.rank < card.rank)
                .map(|other| Self::card_penalty(other, game_status))
                .sum();

            let other_points: i32 = cards.iter()
                .filter(|other| other.suit != card.suit)
                .map(|other| Self::card_penalty(other, game_status))
                .sum();

            let suit_win_points = (Self::chance_of_win(card, game_status) * (suit_points as f32)) as i32;
            let other_win_points = (Self::chance_of_later_win(card, game_status) * (other_points as f32)) as i32;

            dealt_points + suit_win_points + other_win_points
        } else {
            0
        }
    }

    fn later_potential_points(card: &Card, game_status: &GameStatus) -> i32 {
        let cards = Self::remaining_cards(game_status);

        let suit_points: i32 = cards.iter()
            .filter(|other| other.suit == card.suit)
            .filter(|other| other.rank < card.rank)
            .map(|other| Self::card_penalty(other, game_status))
            .sum();

        let other_points: i32 = cards.iter()
            .filter(|other| other.suit != card.suit)
            .map(|other| Self::card_penalty(other, game_status))
            .sum();

        suit_points + ((Self::chance_of_later_win(card, game_status) * (other_points as f32)) as i32)
    }

    fn chance_of_later_win(card: &Card, game_status: &GameStatus) -> f32 {
        let cards = Self::remaining_cards(game_status);
        let suit_cards = cards.iter().filter(|other| other.suit == card.suit).collect::<Vec<_>>();
        let will_win_count = suit_cards.iter().filter(|other| other.rank < card.rank).collect::<Vec<_>>().len();
        if suit_cards.is_empty() {
            1.0
        } else {
            (will_win_count as f32) / (suit_cards.len() as f32)
        }
    }

    fn chance_of_win(card: &Card, game_status: &GameStatus) -> f32 {
        if Self::will_win_deal(card, game_status) {
            1.0
        } else {
            if Self::deal_suit(game_status).map(|suit| suit == &card.suit).unwrap_or(true) && Self::plays_left(game_status).len() > 0 {
                let cards = Self::remaining_cards(game_status);
                let suit_cards = cards.iter().filter(|other| other.suit == card.suit).collect::<Vec<_>>();
                let will_win_count = suit_cards.iter().filter(|other| other.rank < card.rank).collect::<Vec<_>>().len();
                if suit_cards.is_empty() {
                    1.0
                } else {
                    (will_win_count as f32) / (suit_cards.len() as f32)
                }
            } else {
                0.0
            }
        }
    }


}

impl CardStrategy for DefensiveCardStrategy {

    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card> {
        game_status.my_initial_hand.iter()
            .map(|card| ((0 - Self::later_potential_points(card, game_status), card), card))
            .collect::<BTreeMap<_, &Card>>()
            .into_iter()
            .map(|kv| kv.1)
            .take(3)
            .collect()
    }

    fn play_card<'a>(&mut self, game_status: &'a GameStatus, player_name: &PlayerName) -> &'a Card {
        let current_suit = game_status.my_in_progress_deal.as_ref().and_then(|deal| deal.suit);
        let mut valid_cards = game_status.my_current_hand.iter()
            .filter(|card| Some(card.suit) == current_suit)
            .collect::<Vec<&Card>>();

        if valid_cards.is_empty() {
            valid_cards.extend(&game_status.my_current_hand);
        }

        let evaluation = valid_cards.into_iter()
            .map(|card| ((Self::score_card(card, game_status), card), card))
            .collect::<BTreeMap<_,&Card>>();

        // println!("Evaluation: {:?}", evaluation);

        evaluation.values()
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
    fn should_play_heart_1() {
        should_play("should play heart 1", Seven.of(Heart));
    }

    #[test]
    fn should_play_heart_2() {
        should_play("should play heart 2", Four.of(Heart));
    }

    #[test]
    fn should_play_high_rank_1() {
        should_play("should play high rank 1", King.of(Diamond));
    }

    #[test]
    fn should_play_high_rank_2() {
        should_play("should play high rank 2", Ace.of(Spade));
    }

    // #[test]
    // fn should_play_high_rank_3() { // Should get rid of high risk high rank
    //     should_play("should play high rank 3", Ace.of(Spade));
    // }

    #[test]
    fn should_not_crash_during_card_play() {
        should_play("crashed during card play", Ten.of(Heart));
    }

    #[test]
    fn should_play_low_negative_points_card_1() {
        should_play("should play low negative points card 1", Two.of(Diamond));
    }

    #[test]
    fn should_play_high_negative_points_card_1() {
        should_play("should play high negative points card 1", Ace.of(Diamond));
    }

}