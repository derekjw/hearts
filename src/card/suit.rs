use error::Error;
use error::Result;

use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_str = match *self {
            Suit::Heart => "\u{2665}",
            Suit::Diamond => "\u{2666}",
            Suit::Spade => "\u{2660}",
            Suit::Club => "\u{2663}",
        };
        write!(f, "{}", suit_str)
    }
}

impl From<Suit> for &'static str {
    fn from(suit: Suit) -> &'static str {
        match suit {
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
            Suit::Spade => "Spade",
            Suit::Club => "Club"
        }
    }
}

impl FromStr for Suit {
    type Err = Error;

    fn from_str(string: &str) -> Result<Suit> {
        match string {
            "Heart" => Ok(Suit::Heart),
            "Diamond" => Ok(Suit::Diamond),
            "Spade" => Ok(Suit::Spade),
            "Club" => Ok(Suit::Club),
            _ => Err(Error::parsing("Suit", string))
        }
    }
}
