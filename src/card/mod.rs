pub mod dto;
mod suit;
mod rank;

pub use card::suit::Suit;
pub use card::rank::Rank;

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

    use std::str::FromStr;

    #[test]
    fn reversible_suit() {
        assert_eq!(Ok(Suit::Heart), Suit::from_str(Suit::Heart.into()));
    }

    #[test]
    fn reversible_rank() {
        assert_eq!(Ok(Rank::Ace), Rank::from_str(Rank::Ace.into()));
    }

    #[test]
    fn invalid_rank() {
        assert_eq!(Err("Not a valid rank: 1".to_owned()), Rank::from_str("1"));
    }

}
