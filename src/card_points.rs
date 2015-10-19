use card::Card;

#[derive(Debug, PartialEq)]
pub struct CardPoints {
    card: Card,
    points: i32
}

#[cfg(test)]
mod tests {
    use super::*;

	use card::*;

    #[test]
    fn equality() {
    	let card = Card { suit: Suit::Heart, rank: Rank::Ace };
    	assert_eq!(CardPoints { card: card, points: 1 }, CardPoints { card: card, points: 1 });
    }
}
