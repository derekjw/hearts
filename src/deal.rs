use player::PlayerName;
use card::Card;
use card::Suit;

use std::collections::BTreeSet;

pub struct Deal {
    deal_number: u32,
    initiator: PlayerName,
    suit_type: Suit,
    deal_cards: BTreeSet<DealCard>,
    deal_winner: PlayerName,
}

struct DealCard {
    player_name: PlayerName,
    card: Card,
}