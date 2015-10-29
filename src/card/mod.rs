pub mod dto;
mod suit;
mod rank;

pub use card::suit::Suit;
pub use card::rank::Rank;

use serde;
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
                Ok(Some(try!(serializer.visit_struct_elt::<&str>("Suit", self.value.suit.into()))))
            }
            1 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt::<u32>("Number", self.value.rank.into()))))
            }
            2 => {
                self.state += 1;
                Ok(Some(try!(serializer.visit_struct_elt::<&str>("Symbol", self.value.rank.into()))))
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
