use card_strategy::CardStrategy;

use card::Card;
use card::Suit;
use deal::Deal;
use game_status::GameStatus;
use game_status::RoundParameters;
use game_status::GameParticipant;
use player::PlayerName;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct DefensiveCardStrategy;

/*
    Play 2 of clubs if in hand.
*/
impl DefensiveCardStrategy {
    fn score_card<'a>(card: &'a Card, game_status: &'a GameStatus) -> (i32, i32, i32, i32) {
        let remaining_cards = game_status.unplayed_cards();

        let potential_points = Self::potential_points(card, &game_status.game_players, &game_status.in_progress_deal, &remaining_cards, &game_status.round_parameters);
        let definite_points = if Self::will_win_deal(card, &game_status.game_players, &game_status.in_progress_deal, &remaining_cards) {
            potential_points
        } else {
            0
        };
        let later_potential_points = 0 - Self::later_potential_points(card, &remaining_cards, &game_status.round_parameters);
        let card_rank = 0 - (u32::from(card.rank) as i32);
        (definite_points, potential_points, later_potential_points, card_rank)
    }

    fn can_win_deal(card: &Card, in_progress_deal: &Option<Deal>) -> bool {
        in_progress_deal.as_ref().and_then(|deal|
            deal.suit.and_then(|suit|
                deal.deal_cards.iter()
                    .map(|deal_card| deal_card.card)
                    .filter(|card| card.suit == suit)
                    .max()
                    .map(|winning_card| card.suit == suit && card.rank > winning_card.rank))).unwrap_or(true)
    }

    fn will_win_deal(card: &Card, game_players: &Vec<GameParticipant>, in_progress_deal: &Option<Deal>, remaining_cards: &BTreeSet<Card>) -> bool {
        Self::can_win_deal(card, in_progress_deal) && (Self::plays_left(game_players, in_progress_deal).len() == 0 ||
            Self::deal_suit(in_progress_deal)
                .and_then(|suit| remaining_cards.iter()
                    .filter(|card| card.suit == *suit)
                    .max()
                    .map(|winning_card| card.rank > winning_card.rank)).unwrap_or(true))
    }

    fn deal_suit(in_progress_deal: &Option<Deal>) -> Option<&Suit> {
        in_progress_deal.as_ref().and_then(|deal| deal.suit.as_ref())
    }

    fn plays_left<'a>(game_players: &'a Vec<GameParticipant>, in_progress_deal: &Option<Deal>) -> BTreeSet<&'a PlayerName> {
        let mut players: BTreeSet<&PlayerName> = game_players.iter()
            .filter(|player| !player.has_turn)
            .map(|player| &player.team_name)
            .collect();

        if let &Some(ref deal) = in_progress_deal {
            for deal_card in &deal.deal_cards {
                players.remove(&deal_card.player_name);
            }
        }

        players
    }

    fn dealt_cards(in_progress_deal: &Option<Deal>) -> BTreeSet<&Card> {
        in_progress_deal.as_ref()
            .map(|deal| deal.deal_cards.iter().map(|deal_card| &deal_card.card).collect())
            .unwrap_or_default()
    }

    fn potential_points(card: &Card, game_players: &Vec<GameParticipant>, in_progress_deal: &Option<Deal>, remaining_cards: &BTreeSet<Card>, round_parameters: &RoundParameters) -> i32 {
        if Self::can_win_deal(card, in_progress_deal) {
            let dealt_cards = Self::dealt_cards(in_progress_deal);

            let card_points = round_parameters.points(card);

            let dealt_points: i32 = dealt_cards.iter()
                .map(|other| round_parameters.points(other))
                .sum();

            let suit_points: i32 = remaining_cards.iter()
                .filter(|other| other.suit == card.suit)
                .filter(|other| other.rank < card.rank)
                .map(|other| round_parameters.points(other))
                .sum();

            let other_points: i32 = remaining_cards.iter()
                .filter(|other| other.suit != card.suit)
                .map(|other| round_parameters.points(other))
                .sum();

            let number_of_suit = remaining_cards.iter().filter(|other| other.suit == card.suit).map(|_| 1).sum::<u32>();
            let number_dealt = dealt_cards.len() as u32;

            let safe_target = 10 - number_dealt;

            let suit_win_points = if number_dealt < 3 {
                (Self::chance_of_win(card, game_players, in_progress_deal, remaining_cards) * (suit_points as f32)) as i32
            } else {
                0
            };

            let other_win_points = if number_of_suit < safe_target && number_dealt < 3 {
                (Self::chance_of_later_win(card, remaining_cards) * (other_points as f32)) as i32
            } else {
                0
            };

            card_points + dealt_points + suit_win_points + other_win_points
        } else {
            0
        }
    }

    fn later_potential_points(card: &Card, remaining_cards: &BTreeSet<Card>, round_parameters: &RoundParameters) -> i32 {
        let card_points = round_parameters.points(card);

        let suit_points: i32 = remaining_cards.iter()
            .filter(|other| other.suit == card.suit)
            .filter(|other| other.rank < card.rank)
            .map(|other| round_parameters.points(other))
            .sum();

        let other_points: i32 = remaining_cards.iter()
            .filter(|other| other.suit != card.suit)
            .map(|other| round_parameters.points(other))
            .sum();

        let other_win_points = (Self::chance_of_later_win(card, remaining_cards) * (other_points as f32)) as i32;

        card_points + suit_points + other_win_points
    }

    fn chance_of_later_win(card: &Card, remaining_cards: &BTreeSet<Card>) -> f32 {
        let suit_cards = remaining_cards.iter().filter(|other| other.suit == card.suit).collect::<Vec<_>>();
        let will_win_count = suit_cards.iter().filter(|other| other.rank < card.rank).collect::<Vec<_>>().len();
        if suit_cards.is_empty() {
            1.0
        } else {
            (will_win_count as f32) / (suit_cards.len() as f32)
        }
    }

    fn chance_of_win(card: &Card, game_players: &Vec<GameParticipant>, in_progress_deal: &Option<Deal>, remaining_cards: &BTreeSet<Card>) -> f32 {
        if Self::will_win_deal(card, game_players, in_progress_deal, remaining_cards) {
            1.0
        } else {
            if Self::deal_suit(in_progress_deal).map(|suit| suit == &card.suit).unwrap_or(true) && Self::plays_left(game_players, in_progress_deal).len() > 0 {
                let suit_cards = remaining_cards.iter().filter(|other| other.suit == card.suit).collect::<Vec<_>>();
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

    fn pass_card<'a>(hand: &'a BTreeSet<Card>, remaining_cards: &BTreeSet<Card>, round_parameters: &RoundParameters) -> Option<&'a Card> {
        hand.iter()
            .filter(|card| !remaining_cards.contains(card))
            .map(|card| ((0 - Self::later_potential_points(card, &remaining_cards, round_parameters), card), card))
            .collect::<BTreeMap<_, &Card>>()
            .into_iter()
            .map(|kv| kv.1)
            .next()
    }


}

impl CardStrategy for DefensiveCardStrategy {

    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card> {
        let mut remaining_cards = game_status.unplayed_cards();

        let card1 = Self::pass_card(&game_status.my_initial_hand, &remaining_cards, &game_status.round_parameters);
        if let Some(card) = card1 { remaining_cards.insert(card.clone()); };
        let card2 = Self::pass_card(&game_status.my_initial_hand, &remaining_cards, &game_status.round_parameters);
        if let Some(card) = card2 { remaining_cards.insert(card.clone()); };
        let card3 = Self::pass_card(&game_status.my_initial_hand, &remaining_cards, &game_status.round_parameters);

        vec!(card1, card2, card3).into_iter().filter_map(|card| card).collect()
    }

    fn play_card<'a>(&mut self, game_status: &'a GameStatus, player_name: &PlayerName) -> &'a Card {
        let current_suit = game_status.in_progress_deal.as_ref().and_then(|deal| deal.suit);
        let mut valid_cards = game_status.my_current_hand.iter()
            .filter(|card| Some(card.suit) == current_suit)
            .collect::<Vec<&Card>>();

        if valid_cards.is_empty() {
            valid_cards.extend(&game_status.my_current_hand);
        }

        let evaluation = valid_cards.into_iter()
            .map(|card| ((Self::score_card(card, game_status), card), card))
            .collect::<BTreeMap<_,&Card>>();

        // println!("Remaining: {}", game_status.unplayed_cards().iter().map(|card| format!("{}", card)).collect::<Vec<_>>().join(", "));
        // for item in &evaluation {
        //     println!("{}: {:?}", item.1, (item.0).0);
        // }

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

    // #[test]
    // fn should_play_low_negative_points_card_1() {
    //     should_play("should play low negative points card 1", Two.of(Diamond));
    // }

    #[test]
    fn should_play_high_negative_points_card_1() {
        should_play("should play high negative points card 1", Ace.of(Diamond));
    }

    #[test]
    fn should_try_to_win_deal_1() {
        should_play("should try to win deal 1", Ace.of(Club));
    }

}