#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit: suit, rank: rank }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

impl Suit {
    pub fn from_number(num: &u32) -> Option<Suit> {
        match *num {
            1 => Some(Suit::Heart),
            2 => Some(Suit::Diamond),
            3 => Some(Suit::Spade),
            4 => Some(Suit::Club),
            _ => None
        }
    }
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Rank {
    pub fn from_number(num: &u32) -> Option<Rank> {
        match *num {
            1 => Some(Rank::Ace),
            2 => Some(Rank::Two),
            3 => Some(Rank::Three),
            4 => Some(Rank::Four),
            5 => Some(Rank::Five),
            6 => Some(Rank::Six),
            7 => Some(Rank::Seven),
            8 => Some(Rank::Eight),
            9 => Some(Rank::Nine),
            10 => Some(Rank::Ten),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            _ => None
        }
    }
}

impl Into<u32> for Rank {
    fn into(self) -> u32 {
        match self {
            Rank::Ace => 1,
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
            Rank::King => 13
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversible_suit() {
        assert_eq!(Some(Suit::Heart), Suit::from_number(&Suit::Heart.into()));
    }

    #[test]
    fn reversible_rank() {
        assert_eq!(Some(Rank::Ace), Rank::from_number(&Rank::Ace.into()));
    }
}
