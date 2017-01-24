use card::Card;
use card::Suit;
use card::Rank;
use error::Error;
use error::Result;

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
        let suit = Suit::from_str(&dto.suit)?;
        let rank = Rank::from_str(&dto.symbol)?;
        Ok(Card::new(suit, rank))
    }
}

impl <'a> From<&'a Card> for CardDto {
    fn from(card: &'a Card) -> CardDto {
        CardDto {
            suit: card.suit.into(),
            number: card.rank.into(),
            symbol: card.rank.into()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use card::Card;
    use card::Rank;
    use card::Suit;
    use std::convert::TryFrom;

    #[test]
    fn into_card() {
        let dto = CardDto { suit: "Heart".to_owned(), number: 3, symbol: "3".to_owned() };
        let card = Card::try_from(dto);
        assert_eq!(Card::new(Suit::Heart, Rank::Three), card.unwrap());
    }

    #[test]
    fn from_card() {
        let card = Card::new(Suit::Heart, Rank::Three);
        let dto = CardDto::from(&card);
        assert_eq!(CardDto { suit: "Heart".to_owned(), number: 3, symbol: "3".to_owned() }, dto);
    }

}
