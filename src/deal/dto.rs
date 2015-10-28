use player::PlayerName;
use card::Card;
use card::Suit;
use card::dto::CardDto;
use deal::Deal;
use deal::DealCard;
use try_from::TryFrom;
use error::Error;
use error::Result;

use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct DealDto {
    #[serde(rename="DealNumber")]
    deal_number: u32,
    #[serde(rename="Initiator", Default)]
    initiator: Option<PlayerName>,
    #[serde(rename="SuitType")]
    suit_type: String,
    #[serde(rename="DealCards", Default)]
    deal_cards: Vec<DealCardDto>,
    #[serde(rename="DealWinner", Default)]
    deal_winner: Option<PlayerName>,
}

impl TryFrom<DealDto> for Deal {
    type Err = Error;
    fn try_from(dto: DealDto) -> Result<Deal> {
        let deal_cards = try!(dto.deal_cards.into_iter().map(DealCard::try_from).collect::<Result<Vec<DealCard>>>());
        let suit = try!(Suit::from_str(&dto.suit_type));
        Ok(Deal {
            deal_number: dto.deal_number,
            initiator: dto.initiator,
            suit: if deal_cards.is_empty() { None } else { Some(suit) },
            deal_cards: deal_cards,
            deal_winner: dto.deal_winner,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct DealCardDto {
    #[serde(rename="TeamName")]
    team_name: PlayerName,
    #[serde(rename="Card")]
    card: CardDto,
}

impl TryFrom<DealCardDto> for DealCard {
    type Err = Error;
    fn try_from(dto: DealCardDto) -> Result<DealCard> {
        Ok(DealCard {
            player_name: dto.team_name.clone(),
            card: try!(Card::try_from(dto.card))
        })
    }
}
