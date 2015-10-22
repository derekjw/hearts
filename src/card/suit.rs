use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

impl Into<u32> for Suit {
    fn into(self) -> u32 {
        match self {
            Suit::Heart => 1,
            Suit::Diamond => 2,
            Suit::Spade => 3,
            Suit::Club => 4
        }
    }
}

pub struct OptionSuit {
    value: Option<Suit>
}

impl From<u32> for OptionSuit {
    fn from(number: u32) -> OptionSuit {
        let result = match number {
            1 => Some(Suit::Heart),
            2 => Some(Suit::Diamond),
            3 => Some(Suit::Spade),
            4 => Some(Suit::Club),
            _ => None
        };
        OptionSuit { value: result }
    }
}

impl<'a> From<&'a str> for OptionSuit {
    fn from(string: &'a str) -> OptionSuit {
        let result = match string {
            "Heart" => Some(Suit::Heart),
            "Diamond" => Some(Suit::Diamond),
            "Spade" => Some(Suit::Spade),
            "Club" => Some(Suit::Club),
            _ => None
        };
        OptionSuit { value: result }
    }
}

impl Deref for OptionSuit {
    type Target = Option<Suit>;

    fn deref<'a>(&'a self) -> &'a Option<Suit> {
        &self.value
    }
}
