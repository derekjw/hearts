use card::Card;
use card::Suit;
use card::Rank;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct CardDto {
    #[serde(rename="Suit")]
    suit: String,
    #[serde(rename="Number")]
    number: u32,
    #[serde(rename="Symbol")]
    symbol: String,
}

impl From<CardDto> for Result<Card, String> {
    fn from(dto: CardDto) -> Result<Card, String> {
        let suit = try!(Suit::from_str(&dto.suit));
        let rank = try!(Rank::from_str(&dto.symbol));
        Ok(Card::new(suit, rank))
    }
}

impl From<CardDto> for Card {
    fn from(dto: CardDto) -> Card {
        let result: Result<Card, String> = dto.into();
        result.unwrap()
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
        let card: Result<Card, String> = dto.into();
        assert_eq!(Ok(Card::new(Suit::Heart, Rank::Three)), card);
    }

}
