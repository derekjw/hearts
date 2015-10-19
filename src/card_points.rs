use card::Card;

#[derive(Debug, PartialEq)]
pub struct CardPoints {
    card: Card,
    points: i32
}

impl CardPoints {
	pub fn new(card: Card, points: i32) -> CardPoints {
		CardPoints { card: card, points: points }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

	use card::*;

    #[test]
    fn equality() {
    	let card = Card::new(Suit::Heart, Rank::Ace);
    	assert_eq!(CardPoints::new(card, 1), CardPoints::new(card, 1));
    }
}
