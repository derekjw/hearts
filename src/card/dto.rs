use card::Card;
use card::Suit;
use card::Rank;
use error::Error;
use error::Result;
use try_from::TryFrom;
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

impl TryFrom<CardDto> for Card {
    type Err = Error;
    fn try_from(dto: CardDto) -> Result<Card> {
        let suit = try!(Suit::from_str(&dto.suit));
        let rank = try!(Rank::from_str(&dto.symbol));
        Ok(Card::new(suit, rank))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use card::Card;
    use card::Rank;
    use card::Suit;
    use try_from::TryFrom;

    #[test]
    fn into_card() {
        let dto = CardDto { suit: "Heart".to_owned(), number: 3, symbol: "3".to_owned() };
        let card = Card::try_from(dto);
        assert_eq!(Card::new(Suit::Heart, Rank::Three), card.unwrap());
    }

}
