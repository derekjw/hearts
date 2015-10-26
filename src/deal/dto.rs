use player::PlayerName;
use card::Card;
use card::Suit;
use card::dto::CardDto;
use deal::Deal;
use deal::DealCard;

use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct DealDto {
    #[serde(rename="DealNumber")]
    deal_number: u32,
    #[serde(rename="Initiator")]
    initiator: PlayerName,
    #[serde(rename="SuitType")]
    suit_type: String,
    #[serde(rename="DealCards")]
    deal_cards: Vec<DealCardDto>,
    #[serde(rename="DealWinner")]
    deal_winner: PlayerName,
}

impl From<DealDto> for Deal {
    fn from(dto: DealDto) -> Deal {
        Deal {
            deal_number: dto.deal_number,
            initiator: Some(dto.initiator),
            suit: Suit::from_str(&dto.suit_type).unwrap(),
            deal_cards: dto.deal_cards.into_iter().map(DealCard::from).collect(),
            deal_winner: Some(dto.deal_winner),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct InProgressDealDto {
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

impl From<InProgressDealDto> for Deal {
    fn from(dto: InProgressDealDto) -> Deal {
        Deal {
            deal_number: dto.deal_number,
            initiator: dto.initiator,
            suit: Suit::from_str(&dto.suit_type).unwrap(),
            deal_cards: dto.deal_cards.into_iter().map(DealCard::from).collect(),
            deal_winner: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DealCardDto {
    #[serde(rename="TeamName")]
    team_name: PlayerName,
    #[serde(rename="Card")]
    card: CardDto,
}

impl From<DealCardDto> for DealCard {
    fn from(dto: DealCardDto) -> DealCard {
        DealCard {
            player_name: dto.team_name.clone(),
            card: Card::from(dto.card)
        }
    }
}
