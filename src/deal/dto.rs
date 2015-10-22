use player::PlayerName;
use card::Card;
use card::OptionSuit;
use card::dto::CardDto;
use deal::Deal;
use deal::DealCard;

use std::collections::BTreeSet;

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
        let deal_cards: BTreeSet<DealCard> = dto.deal_cards.iter().map(|deal_card_dto| DealCard::from(deal_card_dto)).collect();
        let suit = OptionSuit::from(&dto.suit_type as &str).expect("Invalid suit");
        Deal {
            deal_number: dto.deal_number,
            initiator: dto.initiator,
            suit: suit,
            deal_cards: deal_cards,
            deal_winner: Some(dto.deal_winner),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct InProgressDealDto {
    #[serde(rename="DealNumber")]
    deal_number: u32,
    #[serde(rename="Initiator")]
    initiator: PlayerName,
    #[serde(rename="SuitType")]
    suit_type: String,
    #[serde(rename="DealCards")]
    deal_cards: Vec<DealCardDto>,
}

impl From<InProgressDealDto> for Deal {
    fn from(dto: InProgressDealDto) -> Deal {
        let deal_cards: BTreeSet<DealCard> = dto.deal_cards.iter().map(|deal_card_dto| DealCard::from(deal_card_dto)).collect();
        let suit = OptionSuit::from(&dto.suit_type as &str).expect("Invalid suit");
        Deal {
            deal_number: dto.deal_number,
            initiator: dto.initiator,
            suit: suit,
            deal_cards: deal_cards,
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

impl<'a> From<&'a DealCardDto> for DealCard {
    fn from(dto: &'a DealCardDto) -> DealCard {
        let card = Card::from(&dto.card);
        DealCard { player_name: dto.team_name.clone(), card: card }
    }
}
