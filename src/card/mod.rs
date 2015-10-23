pub mod dto;
mod suit;
mod rank;

pub use card::suit::Suit;
pub use card::suit::OptionSuit;
pub use card::rank::Rank;
pub use card::rank::OptionRank;

use serde;

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

impl serde::Serialize for Card {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: serde::Serializer {
        serializer.visit_struct("Card", CardMapVisitor { value: self, state: 0 })
    }
}

struct CardMapVisitor<'a> {
    value: &'a Card,
    state: u8
}

impl<'a> serde::ser::MapVisitor for CardMapVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
    where S: serde::Serializer {
        match self.state {
            0 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("Suit", &String::from(self.value.suit)))))
            }
            1 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt("Number", &u32::from(self.value.rank)))))
            }
            _ => {
                Ok(None)
            }
        }
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
