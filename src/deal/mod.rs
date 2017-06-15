pub mod dto;

use game_status::PlayerName;
use card::Card;
use card::Suit;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Deal {
    pub deal_number: u32,
    pub initiator: Option<PlayerName>,
    pub suit: Option<Suit>,
    pub deal_cards: Vec<DealCard>,
    pub deal_winner: Option<PlayerName>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct DealCard {
    pub player_name: PlayerName,
    pub card: Card,
}
