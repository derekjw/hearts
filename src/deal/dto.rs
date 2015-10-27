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
    #[serde(rename="Initiator", Default)]
    initiator: Option<PlayerName>,
    #[serde(rename="SuitType")]
    suit_type: String,
    #[serde(rename="DealCards", Default)]
    deal_cards: Vec<DealCardDto>,
    #[serde(rename="DealWinner", Default)]
    deal_winner: Option<PlayerName>,
}

impl From<DealDto> for Deal {
    fn from(dto: DealDto) -> Deal {
        let deal_cards: Vec<DealCard> = dto.deal_cards.into_iter().map(DealCard::from).collect();
        Deal {
            deal_number: dto.deal_number,
            initiator: dto.initiator,
            suit: if deal_cards.is_empty() { None } else { Some(Suit::from_str(&dto.suit_type).unwrap()) },
            deal_cards: deal_cards,
            deal_winner: dto.deal_winner,
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
