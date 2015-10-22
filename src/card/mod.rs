pub mod dto;
mod suit;
mod rank;

pub use card::suit::Suit;
pub use card::suit::OptionSuit;
pub use card::rank::Rank;
pub use card::rank::OptionRank;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversible_suit() {
        assert_eq!(Some(Suit::Heart), *OptionSuit::from(u32::from(Suit::Heart)));
    }

    #[test]
    fn reversible_rank() {
        assert_eq!(Some(Rank::Ace), *OptionRank::from(u32::from(Rank::Ace)));
    }

    #[test]
    fn invalid_rank() {
        assert_eq!(None, *OptionRank::from(1));
    }

}
