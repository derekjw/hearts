use game_status::PlayerName;
use card::Card;
use card::Suit;
use card::dto::CardDto;
use deal::Deal;
use deal::DealCard;
use error::Error;
use error::Result;

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct DealDto {
    #[serde(rename = "DealNumber")]
    deal_number: u32,
    #[serde(rename = "Initiator", default)]
    initiator: Option<PlayerName>,
    #[serde(rename = "SuitType")]
    suit_type: String,
    #[serde(rename = "DealCards", default)]
    deal_cards: Vec<DealCardDto>,
    #[serde(rename = "DealWinner", default)]
    deal_winner: Option<PlayerName>,
}

impl TryFrom<DealDto> for Deal {
    type Error = Error;
    fn try_from(dto: DealDto) -> Result<Deal> {
        let deal_cards = dto.deal_cards
            .into_iter()
            .map(DealCard::try_from)
            .collect::<Result<Vec<DealCard>>>()?;
        let suit = Suit::from_str(&dto.suit_type)?;
        Ok(Deal {
            deal_number: dto.deal_number,
            initiator: dto.initiator,
            suit: if deal_cards.is_empty() {
                None
            } else {
                Some(suit)
            },
            deal_cards: deal_cards,
            deal_winner: dto.deal_winner,
        })
    }
}

impl<'a> From<&'a Deal> for DealDto {
    fn from(deal: &'a Deal) -> DealDto {
        DealDto {
            deal_number: deal.deal_number,
            initiator: deal.initiator.clone(),
            suit_type: deal.suit.unwrap_or(Suit::Club).into(),
            deal_cards: deal.deal_cards.iter().map(DealCardDto::from).collect(),
            deal_winner: deal.deal_winner.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DealCardDto {
    #[serde(rename = "TeamName")]
    team_name: PlayerName,
    #[serde(rename = "Card")]
    card: CardDto,
}

impl TryFrom<DealCardDto> for DealCard {
    type Error = Error;
    fn try_from(dto: DealCardDto) -> Result<DealCard> {
        Ok(DealCard {
            player_name: dto.team_name.clone(),
            card: Card::try_from(dto.card)?,
        })
    }
}

impl<'a> From<&'a DealCard> for DealCardDto {
    fn from(deal_card: &'a DealCard) -> DealCardDto {
        DealCardDto {
            team_name: deal_card.player_name.clone(),
            card: CardDto::from(&deal_card.card),
        }
    }
}
