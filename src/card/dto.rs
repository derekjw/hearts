use card::Card;
use card::OptionSuit;
use card::OptionRank;

#[derive(Deserialize, Debug)]
pub struct CardDto {
    #[serde(rename="Suit")]
    suit: String,
    #[serde(rename="Number")]
    number: u32,
    #[serde(rename="Symbol")]
    symbol: String,
}

impl From<CardDto> for Card {
    fn from(dto: CardDto) -> Card {
        let suit = OptionSuit::from(&dto.suit as &str).expect("Not a valid suit");
        let rank = OptionRank::from(dto.number).expect("Not a valid number");
        Card::new(suit, rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use card::Card;
    use card::Rank;
    use card::Suit;

    #[test]
    fn into_card() {
        let dto = CardDto { suit: "Heart".to_owned(), number: 3, symbol: "3".to_owned() };
        let card: Card = dto.into();
        assert_eq!(Card::new(Suit::Heart, Rank::Three), card);
    }

}
