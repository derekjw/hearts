use std::fmt;

string_enum! {
    Suit {
        Spade,
        Heart,
        Diamond,
        Club,
    }
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
