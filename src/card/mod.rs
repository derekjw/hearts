pub mod dto;
mod suit;
mod rank;

pub use card::suit::Suit;
pub use card::rank::Rank;

use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit: suit, rank: rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;
    use std::error::Error;

    #[test]
    fn reversible_suit() {
        assert_eq!(Suit::Heart, Suit::from_str(Suit::Heart.into()).unwrap());
    }

    #[test]
    fn reversible_rank() {
        assert_eq!(Rank::Ace, Rank::from_str(Rank::Ace.into()).unwrap());
    }

    #[test]
    fn invalid_rank() {
        assert_eq!("Error while parsing \"1\" as Rank", Rank::from_str("1").unwrap_err().description());
    }

}
