pub mod dto;

use player::PlayerName;
use card::Card;
use card::Suit;

use std::collections::BTreeSet;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Deal {
    pub deal_number: u32,
    pub initiator: PlayerName,
    pub suit: Suit,
    pub deal_cards: BTreeSet<DealCard>,
    pub deal_winner: Option<PlayerName>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct DealCard {
    pub player_name: PlayerName,
    pub card: Card,
}