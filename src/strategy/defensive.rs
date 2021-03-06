use strategy::CardStrategy;

use card::Card;
use card::Suit;
use card::Rank;
use deal::Deal;
use game_status::GameStatus;
use game_status::RoundParameters;
use game_status::GameParticipant;
use game_status::PlayerName;

use std::fmt;
use std::iter;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct DefensiveCardStrategy {
    player_name: PlayerName,
    shooting_the_moon: bool,
}

impl DefensiveCardStrategy {
    pub fn new(player_name: PlayerName) -> DefensiveCardStrategy {
        DefensiveCardStrategy {
            player_name: player_name,
            shooting_the_moon: false,
        }
    }

    fn score_card(&self, card: &Card, game_status: &GameStatus) -> CardScore {
        let remaining_cards = game_status.unplayed_cards();

        let void_suits = Self::void_suits(game_status);

        let plays_left = Self::plays_left(&game_status.game_players, &game_status.in_progress_deal);

        let deal_void_suits = Self::deal_void_suits(&void_suits, &plays_left);

        let safe_remaining_cards_iter = game_status.unplayed_cards().into_iter().filter(|other| {
            !deal_void_suits.contains(&other.suit)
        });

        let safe_remaining_cards =
            if game_status.in_progress_deal.as_ref().map_or(true, |deal| {
                deal.deal_cards.is_empty()
            })
            {
                safe_remaining_cards_iter.collect::<BTreeSet<_>>()
            } else {
                safe_remaining_cards_iter
                    .filter(|other| !game_status.cards_passed_by_me.contains(other))
                    .collect()
            };

        let potential_points = self.potential_points(
            card,
            &game_status.game_players,
            &game_status.in_progress_deal,
            &safe_remaining_cards,
            &void_suits,
            &game_status.round_parameters,
        );

        let definite_points = if Self::will_win_deal(
            card,
            &game_status.game_players,
            &game_status.in_progress_deal,
            &safe_remaining_cards,
        )
        {
            potential_points
        } else {
            0.0
        };

        let later_potential_points = 0.0 -
            Self::later_potential_points(card, &remaining_cards, &game_status.round_parameters);

        let rank_modifier = if game_status.round_parameters.points(card) < 0 {
            -1
        } else {
            1
        };

        let card_rank = 0 - (u32::from(card.rank) as i32) * rank_modifier;

        CardScore {
            definite_points: (definite_points * 1000.0) as i32,
            potential_points: (potential_points * 1000.0) as i32,
            later_potential_points: (later_potential_points * 1000.0) as i32,
            rank: card_rank,
        }
    }

    fn possible_shooter<'a>(
        game_players: &'a [GameParticipant],
        in_progress_deal: &Option<Deal>,
        deals: &[Deal],
        round_parameters: &RoundParameters,
    ) -> Option<&'a GameParticipant> {
        let possible_shooters = Self::possible_shooters(game_players, deals, round_parameters);

        let shoot_target = 20 - deals.len() as i32;

        if possible_shooters.len() == 1 {
            possible_shooters
                .into_iter()
                .filter(|&(_, shoot_score)| shoot_score > shoot_target)
                .map(|(player, _)| player)
                .filter(|shooter| {
                    in_progress_deal.as_ref().map_or(true, |deal| {
                        Self::player_might_win_deal(&shooter.team_name, deal)
                    })
                })
                .next()
        } else {
            None
        }
    }

    fn player_might_win_deal(player: &PlayerName, in_progress_deal: &Deal) -> bool {
        in_progress_deal
            .suit
            .and_then(|suit| {
                in_progress_deal
                    .deal_cards
                    .iter()
                    .filter(|deal_card| deal_card.player_name == *player)
                    .map(|deal_card| deal_card.card)
                    .map(|shooter_card| {
                        shooter_card.suit == suit &&
                            in_progress_deal
                                .deal_cards
                                .iter()
                                .filter(|deal_card| deal_card.card.suit == suit)
                                .filter(|deal_card| deal_card.card.rank > shooter_card.rank)
                                .next()
                                .is_none()
                    })
                    .next()
            })
            .unwrap_or(true)
    }

    fn possible_shooters<'a>(
        game_players: &'a [GameParticipant],
        deals: &[Deal],
        round_parameters: &RoundParameters,
    ) -> Vec<(&'a GameParticipant, i32)> {
        game_players
            .iter()
            .map(|player| {
                (
                    player,
                    Self::cards_won(deals, &player.team_name)
                        .iter()
                        .filter(|card| Self::shooting_card(card))
                        .map(|card| round_parameters.points(card))
                        .sum::<i32>(),
                )
            })
            .filter(|&(_, shoot_score)| shoot_score > 0)
            .collect()
    }

    fn am_i_shooter(&self, game_status: &GameStatus, multiplier: f32) -> bool {
        let possible_shooters = Self::possible_shooters(
            &game_status.game_players,
            &game_status.game_deals,
            &game_status.round_parameters,
        );
        let remaining_cards = game_status.unplayed_cards();
        let hand = &game_status.my_current_hand;

        possible_shooters.len() <= 1 &&
            possible_shooters
                .into_iter()
                .map(|(shooter, _)| shooter.team_name == self.player_name)
                .next()
                .unwrap_or(true) &&
            (self.shooting_the_moon ||
                 (hand.iter()
                      .filter(|card| {
                    Self::will_win_deal(
                        card,
                        &game_status.game_players,
                        &game_status.in_progress_deal,
                        &remaining_cards,
                    )
                })
                      .collect::<Vec<_>>()
                      .len() as f32 * multiplier > hand.len() as f32))
    }

    fn cards_won<'a>(deals: &'a [Deal], player: &PlayerName) -> BTreeSet<&'a Card> {
        deals
            .iter()
            .filter(|deal| {
                deal.deal_winner.as_ref().map_or(
                    false,
                    |winner| winner == player,
                )
            })
            .flat_map(|deal| deal.deal_cards.iter())
            .map(|deal_card| &deal_card.card)
            .collect()
    }

    fn shooting_card(card: &Card) -> bool {
        card.suit == Suit::Heart || (card.suit == Suit::Spade && card.rank == Rank::Queen)
    }

    fn deal_void_suits(
        void_suits: &BTreeMap<&PlayerName, BTreeSet<Suit>>,
        plays_left: &BTreeSet<&PlayerName>,
    ) -> BTreeSet<Suit> {
        void_suits
            .iter()
            .filter(|&(player, _)| plays_left.contains(player))
            .map(|(_, suits)| suits)
            .fold(None as Option<BTreeSet<Suit>>, |option_result, suits| {
                option_result
                    .map(|result| result.intersection(suits).cloned().collect())
                    .or_else(|| Some(suits.clone()))
            })
            .unwrap_or_default()
    }

    fn void_suits(game_status: &GameStatus) -> BTreeMap<&PlayerName, BTreeSet<Suit>> {
        game_status
            .game_players
            .iter()
            .map(|player| {
                (
                    &player.team_name,
                    Self::player_void_suits(game_status, &player.team_name),
                )
            })
            .collect()

    }

    fn player_void_suits(game_status: &GameStatus, player_name: &PlayerName) -> BTreeSet<Suit> {
        game_status
            .game_deals
            .iter()
            .filter_map(|deal| {
                deal.suit.and_then(|suit| {
                    deal.deal_cards
                        .iter()
                        .filter(|deal_card| {
                            &deal_card.player_name == player_name && deal_card.card.suit != suit
                        })
                        .map(|_| suit)
                        .next()
                })
            })
            .collect()
    }

    fn can_win_deal(card: &Card, in_progress_deal: &Option<Deal>) -> bool {
        in_progress_deal
            .as_ref()
            .and_then(|deal| {
                let suit = &deal.suit.unwrap_or(card.suit);
                deal.deal_cards
                    .iter()
                    .map(|deal_card| &deal_card.card)
                    .filter(|other| &other.suit == suit)
                    .max()
                    .map(|winning_card| {
                        &card.suit == suit && card.rank > winning_card.rank
                    })
            })
            .unwrap_or(true)
    }

    fn will_win_deal(
        card: &Card,
        game_players: &[GameParticipant],
        in_progress_deal: &Option<Deal>,
        remaining_cards: &BTreeSet<Card>,
    ) -> bool {
        Self::can_win_deal(card, in_progress_deal) &&
            (Self::plays_left(game_players, in_progress_deal).is_empty() ||
                 {
                     let suit = Self::deal_suit(in_progress_deal).unwrap_or(&card.suit);
                     remaining_cards
                         .iter()
                         .filter(|other| &other.suit == suit)
                         .max()
                         .map_or(true, |winning_card| card.rank > winning_card.rank)
                 })
    }

    fn deal_suit(in_progress_deal: &Option<Deal>) -> Option<&Suit> {
        in_progress_deal.as_ref().and_then(
            |deal| deal.suit.as_ref(),
        )
    }

    fn plays_left<'a>(
        game_players: &'a [GameParticipant],
        in_progress_deal: &Option<Deal>,
    ) -> BTreeSet<&'a PlayerName> {
        let mut players: BTreeSet<&PlayerName> = game_players
            .iter()
            .filter(|player| !player.has_turn)
            .map(|player| &player.team_name)
            .collect();

        if let Some(ref deal) = *in_progress_deal {
            for deal_card in &deal.deal_cards {
                players.remove(&deal_card.player_name);
            }
        }

        players
    }

    fn dealt_cards(in_progress_deal: &Option<Deal>) -> BTreeSet<&Card> {
        in_progress_deal
            .as_ref()
            .map(|deal| {
                deal.deal_cards
                    .iter()
                    .map(|deal_card| &deal_card.card)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn potential_points(
        &self,
        card: &Card,
        game_players: &[GameParticipant],
        in_progress_deal: &Option<Deal>,
        remaining_cards: &BTreeSet<Card>,
        void_suits: &BTreeMap<&PlayerName, BTreeSet<Suit>>,
        round_parameters: &RoundParameters,
    ) -> f32 {
        if Self::can_win_deal(card, in_progress_deal) {
            let dealt_cards = Self::dealt_cards(in_progress_deal);

            let card_points = round_parameters.points(card) as f32;

            let dealt_points = dealt_cards
                .iter()
                .map(|other| round_parameters.points(other))
                .sum::<i32>() as f32;

            let suit_points = remaining_cards
                .iter()
                .chain(iter::once(card))
                .filter(|other| other.suit == card.suit)
                .filter(|other| other.rank <= card.rank)
                .map(|other| round_parameters.points(other))
                .sum::<i32>() as f32;

            let other_points = remaining_cards
                .iter()
                .filter(|other| other.suit != card.suit)
                .map(|other| round_parameters.points(other))
                .filter(|points| points > &0)
                .sum::<i32>() as f32;

            let number_of_suit = remaining_cards
                .iter()
                .filter(|other| other.suit == card.suit)
                .map(|_| 1)
                .sum::<i32>();
            let number_dealt = dealt_cards.len();

            let safe_target = 9.0 + card_points + dealt_points - (number_dealt as f32);

            let suit_win_modifier =
                if suit_points < 0.0 && dealt_points > 2.0 && number_dealt < 3 {
                    0.0 - 0.5
                } else {
                    1.0
                };

            let suit_win_points = if number_dealt < 3 {
                Self::chance_of_win(card, game_players, in_progress_deal, remaining_cards) *
                    suit_points * suit_win_modifier
            } else {
                0.0
            };

            let plays_left = Self::plays_left(game_players, in_progress_deal);

            let voider = void_suits
                .iter()
                .filter(|&(player_name, ref player_void_suits)| {
                    plays_left.contains(player_name) && player_void_suits.contains(&card.suit)
                })
                .next()
                .is_some();

            let other_win_points = if self.shooting_the_moon || voider ||
                ((number_of_suit as f32) < safe_target && number_dealt < 3)
            {
                let other_cards = remaining_cards
                    .iter()
                    .chain(dealt_cards)
                    .cloned()
                    .collect::<BTreeSet<_>>();
                Self::chance_of_later_win(card, &other_cards) * other_points
            } else {
                0.0
            };

            dealt_points + suit_win_points + other_win_points
        } else {
            0.0
        }
    }

    fn later_potential_points(
        card: &Card,
        remaining_cards: &BTreeSet<Card>,
        round_parameters: &RoundParameters,
    ) -> f32 {
        let card_points = round_parameters.points(card) as f32;

        let suit_points = remaining_cards
            .iter()
            .filter(|other| other.suit == card.suit)
            .filter(|other| other.rank < card.rank)
            .map(|other| round_parameters.points(other))
            .sum::<i32>() as f32;

        let other_points = remaining_cards
            .iter()
            .filter(|other| other.suit != card.suit)
            .map(|other| round_parameters.points(other))
            .filter(|points| points > &0)
            .sum::<i32>() as f32;

        let other_win_points = Self::chance_of_later_win(card, remaining_cards) * other_points;

        card_points + suit_points + other_win_points
    }

    fn chance_of_later_win(card: &Card, remaining_cards: &BTreeSet<Card>) -> f32 {
        let suit_cards = remaining_cards
            .iter()
            .filter(|other| other.suit == card.suit)
            .collect::<Vec<_>>();
        let will_win_count = suit_cards
            .iter()
            .filter(|other| other.rank < card.rank)
            .collect::<Vec<_>>()
            .len();
        if suit_cards.is_empty() {
            1.0
        } else {
            (will_win_count as f32) / (suit_cards.len() as f32)
        }
    }

    fn chance_of_win(
        card: &Card,
        game_players: &[GameParticipant],
        in_progress_deal: &Option<Deal>,
        remaining_cards: &BTreeSet<Card>,
    ) -> f32 {
        if Self::will_win_deal(card, game_players, in_progress_deal, remaining_cards) {
            1.0
        } else {
            if Self::deal_suit(in_progress_deal).map_or(true, |suit| suit == &card.suit) &&
                !Self::plays_left(game_players, in_progress_deal).is_empty()
            {
                let suit_cards = remaining_cards
                    .iter()
                    .chain(
                        in_progress_deal
                            .iter()
                            .flat_map(|deal| deal.deal_cards.iter())
                            .map(|deal_card| &deal_card.card),
                    )
                    .filter(|other| other.suit == card.suit)
                    .collect::<Vec<_>>();
                let will_win_count = suit_cards
                    .iter()
                    .filter(|other| other.rank < card.rank)
                    .collect::<Vec<_>>()
                    .len();
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

    fn pass_card<'a>(
        hand: &'a BTreeSet<Card>,
        remaining_cards: &BTreeSet<Card>,
        round_parameters: &RoundParameters,
        shooting: bool,
    ) -> Option<&'a Card> {
        hand.iter()
            .filter(|card| !remaining_cards.contains(card))
            .map(|card| {
                let mut points = 0 -
                    (Self::later_potential_points(card, &remaining_cards, round_parameters) *
                         1000.0) as i32;
                let mut rank = u32::from(card.rank) as i32;
                if card.suit == Suit::Heart {
                    points = points - 2 // higher priority on hearts
                }
                if card.suit == Suit::Spade && card.rank > Rank::Jack {
                    points = points - 2 // higher priority on queen of spades (and friends)
                }
                if shooting {
                    points = points.abs()
                }
                if !shooting {
                    rank = 0 - rank;
                }
                ((points, rank), card)
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|(_, card)| card)
            .next()
    }
}

impl CardStrategy for DefensiveCardStrategy {
    fn player_name(&self) -> &PlayerName {
        &self.player_name
    }

    fn pass_cards<'a>(&mut self, game_status: &'a GameStatus) -> Vec<&'a Card> {
        self.shooting_the_moon = false;

        info!(
            "My Hand : {}",
            game_status
                .my_current_hand
                .iter()
                .map(|card| format!("{}", card))
                .collect::<Vec<String>>()
                .join(" ")
        );
        let mut remaining_cards = game_status.unplayed_cards();

        let i_am_shooter = self.am_i_shooter(game_status, 2.5);
        self.shooting_the_moon = i_am_shooter;

        let card1 = Self::pass_card(
            &game_status.my_initial_hand,
            &remaining_cards,
            &game_status.round_parameters,
            i_am_shooter,
        );
        if let Some(card) = card1 {
            remaining_cards.insert(card.clone());
        };
        let card2 = Self::pass_card(
            &game_status.my_initial_hand,
            &remaining_cards,
            &game_status.round_parameters,
            i_am_shooter,
        );
        if let Some(card) = card2 {
            remaining_cards.insert(card.clone());
        };
        let card3 = Self::pass_card(
            &game_status.my_initial_hand,
            &remaining_cards,
            &game_status.round_parameters,
            i_am_shooter,
        );

        vec![card1, card2, card3]
            .into_iter()
            .filter_map(|card| card)
            .collect()
    }

    fn play_card<'a>(&mut self, game_status: &'a GameStatus) -> &'a Card {
        let two_of_clubs = Rank::Two.of(Suit::Club);
        if let Some(card) = game_status.my_current_hand.iter().find(|&card| {
            card == &two_of_clubs
        })
        {
            card
        } else {
            let current_suit = game_status.in_progress_deal.as_ref().and_then(
                |deal| deal.suit,
            );
            let mut valid_cards = game_status
                .my_current_hand
                .iter()
                .filter(|card| Some(card.suit) == current_suit)
                .collect::<Vec<&Card>>();

            if valid_cards.is_empty() {
                valid_cards.extend(&game_status.my_current_hand);
            }

            let initial_evaluation = valid_cards
                .into_iter()
                .map(|card| (self.score_card(card, game_status), card))
                .collect::<BTreeSet<_>>();

            let possible_shooter = Self::possible_shooter(
                &game_status.game_players,
                &game_status.in_progress_deal,
                &game_status.game_deals,
                &game_status.round_parameters,
            );

            let i_am_shooter = self.am_i_shooter(game_status, 2.0);
            self.shooting_the_moon = i_am_shooter;

            if i_am_shooter {
                info!("Shooting the moon!");
            } else if let Some(shooter) = possible_shooter {
                info!("Possible shooter detected: {}", shooter.team_name);
            }

            let evaluation = if i_am_shooter || possible_shooter.is_some() {
                initial_evaluation
                    .into_iter()
                    .map(|(card_score, card)| (card_score.invert(), card))
                    .collect()
            } else {
                initial_evaluation
            };

            info!(
                "Unplayed: {}",
                game_status
                    .unplayed_cards()
                    .iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            info!(
                "Void: {}",
                Self::void_suits(game_status)
                    .iter()
                    .map(|(player_name, ref player_void_suits)| {
                        format!(
                            "{}[{}]",
                            player_name,
                            player_void_suits
                                .iter()
                                .map(|suit| format!("{}", suit))
                                .collect::<Vec<_>>()
                                .join("")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            info!(
                "My Hand:  {}",
                game_status
                    .my_current_hand
                    .iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            for item in &evaluation {
                let &(ref score, ref card) = item;
                info!("{}: {}", card, score);
            }

            evaluation.iter().map(|&(_, card)| card).next().expect(
                "No valid cards to play!",
            )
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct CardScore {
    definite_points: i32,
    potential_points: i32,
    later_potential_points: i32,
    rank: i32,
}

impl CardScore {
    pub fn invert(&self) -> CardScore {
        CardScore {
            definite_points: 0 - self.definite_points.abs(),
            potential_points: 0 - self.potential_points.abs(),
            later_potential_points: self.later_potential_points.abs(),
            rank: self.rank,
        }
    }
}

impl fmt::Display for CardScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{: >7.3}, {: >7.3}, {: >7.3}, {: >3.2}",
            self.definite_points as f32 / 1000.0,
            self.potential_points as f32 / 1000.0,
            self.later_potential_points as f32 / 1000.0,
            self.rank
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_status::PlayerName;
    use card::Card;
    use card::Rank::*;
    use card::Suit::*;
    use game_status::GameStatus;
    use game_status::dto::GameStatusDto;
    use strategy::CardStrategy;

    use std::convert::TryFrom;
    use error::Error;

    extern crate serde;
    extern crate serde_json;

    use std::fs::File;
    use std::io::Read;

    fn open_scenario(name: &str) -> GameStatus {
        let file_name = format!("samples/scenarios/{}.json", name.replace("_", " "));
        let mut game_status_file = File::open(file_name).unwrap();
        let mut game_status_string = String::new();
        game_status_file
            .read_to_string(&mut game_status_string)
            .unwrap();
        let game_status_dto: GameStatusDto = serde_json::from_str(&game_status_string)
            .map_err(Error::from)
            .unwrap();
        GameStatus::try_from(game_status_dto).unwrap()
    }

    fn should_play(name: &str, expected_card: Card) {
        let player_name = PlayerName::new("Derek Williams");
        let game_status = open_scenario(name);
        let card = DefensiveCardStrategy::new(player_name)
            .play_card(&game_status)
            .clone();
        assert_eq!(expected_card, card);
    }

    macro_rules! test_play {
        ($($name:ident => $card:expr)*) => {
            $(#[test]
            fn $name() {
                should_play(stringify!($name), $card);
            })*
        }
    }

    test_play! {
        normal_1 => Jack.of(Diamond)
        normal_2 => King.of(Club)
        normal_3 => Six.of(Spade)

        should_play_heart_1 => Seven.of(Heart)
        should_play_heart_2 => Four.of(Heart)
        should_not_play_high_heart_1 => Seven.of(Heart)
        should_play_high_rank_1 => King.of(Diamond)
        should_play_high_rank_2 => Ace.of(Spade)
        should_play_high_rank_3 => Ace.of(Spade)
        should_not_crash_during_card_play => Ten.of(Heart)
        // should_play_low_negative_points_card_1 => Two.of(Diamond)
        should_play_high_negative_points_card_1 => Ace.of(Diamond)
        should_play_high_negative_points_card_2 => King.of(Diamond)
        should_try_to_win_deal_1 => Queen.of(Club)
        should_prevent_shooter_1 => King.of(Heart)
        // should_play_low_club_1 => Four.of(Club) // To the moon!!
        should_play_low_club_2 => Three.of(Club)
        should_play_low_heart_1 => Two.of(Heart)
        should_not_play_queen_of_spades => Ten.of(Spade)

        // corrections to this game cause no difference to outcome
        normal_game_1_01_01 => Four.of(Club)
        normal_game_1_01_02 => Eight.of(Heart)
        normal_game_1_01_03 => Seven.of(Club) // Six.of(Club)
        normal_game_1_01_04 => Five.of(Spade)
        normal_game_1_01_05 => Ten.of(Club) // Seven.of(Club)
        normal_game_1_01_06 => Seven.of(Heart)
        normal_game_1_01_07 => Six.of(Heart)
        normal_game_1_01_08 => Four.of(Diamond)
        normal_game_1_01_09 => Ten.of(Club)
        normal_game_1_01_10 => Nine.of(Club)
        normal_game_1_01_11 => Three.of(Heart)
        normal_game_1_01_12 => Four.of(Spade)
        normal_game_1_01_13 => Two.of(Spade)

        // corrections to this game cause no difference to outcome
        normal_game_1_02_01 => Eight.of(Club) // Six of Club
        normal_game_1_02_02 => Queen.of(Diamond)
        normal_game_1_02_03 => Eight.of(Club)
        normal_game_1_02_04 => Ace.of(Heart)
        normal_game_1_02_05 => Three.of(Heart)
        normal_game_1_02_06 => Eight.of(Diamond)
        normal_game_1_02_07 => Five.of(Diamond)
        normal_game_1_02_08 => Nine.of(Spade)
        normal_game_1_02_09 => Eight.of(Spade)
        normal_game_1_02_10 => Seven.of(Spade)
        normal_game_1_02_11 => Six.of(Spade)
        normal_game_1_02_12 => Three.of(Spade)
        normal_game_1_02_13 => Two.of(Spade)
    }

}
