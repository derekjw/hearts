use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
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

impl From<Suit> for String {
    fn from(suit: Suit) -> String {
        let suit_str: &str = suit.into();
        suit_str.to_owned()
    }
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(string: &str) -> Result<Suit, String> {
        match string {
            "Heart" => Ok(Suit::Heart),
            "Diamond" => Ok(Suit::Diamond),
            "Spade" => Ok(Suit::Spade),
            "Club" => Ok(Suit::Club),
            other => Err(format!("Not a valid suit: {}", other))
        }
    }
}
