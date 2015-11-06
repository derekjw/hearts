use card::Card;
use card::Suit;

use error::Result;

use std::fmt;

string_enum! {
    Rank {
        Two => 2,
        Three => 3,
        Four => 4,
        Five => 5,
        Six => 6,
        Seven => 7,
        Eight => 8,
        Nine => 9,
        Ten => 10,
        Jack => J,
        Queen => Q,
        King => K,
        Ace => A,
    }
}

impl Rank {
    pub fn of(self, suit: Suit) -> Card {
        Card::new(suit, self)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match *self {
            Rank::Ten => "T",
            other => other.into(),
        };
        write!(f, "{}", rank_str)
    }
}

impl From<Rank> for u32 {
    fn from(rank: Rank) -> u32 {
        match rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14
        }
    }
}
