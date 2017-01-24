use game_status::RoundParameters;
use game_status::GameStatus;
use game_status::GameInstanceState;
use game_status::RoundState;
use game_status::HeartsGameInstanceState;
use game_status::GameParticipant;
use game_status::PlayerName;

use card::Card;
use card::dto::CardDto;
use deal::Deal;
use deal::dto::DealDto;
use try_from::TryFrom;
use try_from::TryInto;
use error::Error;
use error::Result;

use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameStatusDto {
    #[serde(rename="CurrentGameId")]
    current_game_id: String,
    #[serde(rename="CurrentGameState")]
    current_game_state: String,
    #[serde(rename="CurrentRoundId")]
    current_round_id: u32,
    #[serde(rename="CurrentRoundState")]
    current_round_state: String,
    #[serde(rename="RoundParameters", default)]
    round_parameters: RoundParametersDto,
    #[serde(rename="MyGameState")]
    my_game_state: String,
    #[serde(rename="MyGameStateDescription")]
    my_game_state_description: String,
    #[serde(rename="MyGameParticipants", default)]
    my_game_participants: Vec<GameParticipantDto>,
    #[serde(rename="MyInitialHand", default)]
    my_initial_hand: Vec<CardDto>,
    #[serde(rename="CardsPassedByMe", default)]
    cards_passed_by_me: Vec<CardDto>,
    #[serde(rename="CardsPassedToMe", default)]
    cards_passed_to_me: Vec<CardDto>,
    #[serde(rename="MyFinalHand", default)]
    my_final_hand: Vec<CardDto>,
    #[serde(rename="MyCurrentHand", default)]
    my_current_hand: Vec<CardDto>,
    #[serde(rename="MyGameDeals", default)]
    my_game_deals: Vec<DealDto>,
    #[serde(rename="MyInProgressDeal", default)]
    my_in_progress_deal: Option<DealDto>,
    #[serde(rename="IsMyTurn")]
    is_my_turn: bool,
}

impl TryFrom<GameStatusDto> for GameStatus {
    type Err = Error;

    fn try_from(dto: GameStatusDto) -> Result<GameStatus> {
        let my_in_progress_deal = match dto.my_in_progress_deal.map(Deal::try_from) {
            Some(result) => result.map(Some),
            None => Ok(None)
        };
        Ok(GameStatus {
            current_game_id: dto.current_game_id,
            current_game_state: GameInstanceState::from_str(&dto.current_game_state)?,
            current_round_id: dto.current_round_id,
            current_round_state: RoundState::from_str(&dto.current_round_state)?,
            round_parameters: RoundParameters::try_from(dto.round_parameters)?,
            game_state: HeartsGameInstanceState::from_str(&dto.my_game_state)?,
            game_state_description: dto.my_game_state_description,
            game_players: dto.my_game_participants.into_iter().map(GameParticipant::from).collect(),
            my_initial_hand: dto.my_initial_hand.into_iter().map(Card::try_from).collect::<Result<_>>()?,
            cards_passed_by_me: dto.cards_passed_by_me.into_iter().map(Card::try_from).collect::<Result<_>>()?,
            cards_passed_to_me: dto.cards_passed_to_me.into_iter().map(Card::try_from).collect::<Result<_>>()?,
            my_final_hand: dto.my_final_hand.into_iter().map(Card::try_from).collect::<Result<_>>()?,
            my_current_hand: dto.my_current_hand.into_iter().map(Card::try_from).collect::<Result<_>>()?,
            game_deals: dto.my_game_deals.into_iter().map(Deal::try_from).collect::<Result<_>>()?,
            in_progress_deal: my_in_progress_deal?,
            is_my_turn: dto.is_my_turn,
        })
    }
}

impl <'a> From<&'a GameStatus> for GameStatusDto {
    fn from(game_status: &'a GameStatus) -> GameStatusDto {
        GameStatusDto {
            current_game_id: game_status.current_game_id.clone(),
            current_game_state: String::from(&game_status.current_game_state),
            current_round_id: game_status.current_round_id,
            current_round_state: String::from(&game_status.current_round_state),
            round_parameters: RoundParametersDto::from(&game_status.round_parameters),
            my_game_state: String::from(&game_status.game_state),
            my_game_state_description: game_status.game_state_description.clone(),
            my_game_participants: game_status.game_players.iter().map(GameParticipantDto::from).collect(),
            my_initial_hand: game_status.my_initial_hand.iter().map(CardDto::from).collect(),
            cards_passed_by_me: game_status.cards_passed_by_me.iter().map(CardDto::from).collect(),
            cards_passed_to_me: game_status.cards_passed_to_me.iter().map(CardDto::from).collect(),
            my_final_hand: game_status.my_final_hand.iter().map(CardDto::from).collect(),
            my_current_hand: game_status.my_current_hand.iter().map(CardDto::from).collect(),
            my_game_deals: game_status.game_deals.iter().map(DealDto::from).collect(),
            my_in_progress_deal: game_status.in_progress_deal.as_ref().map(DealDto::from),
            is_my_turn: game_status.is_my_turn,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RoundParametersDto {
    #[serde(rename="RoundId")]
    round_id: u32,
    #[serde(rename="InitiationPhaseInSeconds")]
    initiation_phase_in_seconds: u32,
    #[serde(rename="PassingPhaseInSeconds")]
    passing_phase_in_seconds: u32,
    #[serde(rename="DealingPhaseInSeconds")]
    dealing_phase_in_seconds: u32,
    #[serde(rename="FinishingPhaseInSeconds")]
    finishing_phase_in_seconds: u32,
    #[serde(rename="NumberOfCardsTobePassed")]
    number_of_cards_to_be_passed: u32,
    #[serde(rename="CardPoints")]
    card_points: Vec<CardPointsDto>
}

impl TryFrom<RoundParametersDto> for RoundParameters {
    type Err = Error;
    fn try_from(dto: RoundParametersDto) -> Result<RoundParameters> {
        Ok(RoundParameters {
            round_id: dto.round_id,
            initiation_phase_in_seconds: dto.initiation_phase_in_seconds,
            passing_phase_in_seconds: dto.passing_phase_in_seconds,
            dealing_phase_in_seconds: dto.dealing_phase_in_seconds,
            finishing_phase_in_seconds: dto.finishing_phase_in_seconds,
            number_of_cards_to_be_passed: dto.number_of_cards_to_be_passed,
            card_points: dto.card_points.into_iter().map(CardPointsDto::try_into).collect::<Result<_>>()?,
        })
    }
}

impl <'a> From<&'a RoundParameters> for RoundParametersDto {
    fn from(entity: &'a RoundParameters) -> RoundParametersDto {
        RoundParametersDto {
            round_id: entity.round_id,
            initiation_phase_in_seconds: entity.initiation_phase_in_seconds,
            passing_phase_in_seconds: entity.passing_phase_in_seconds,
            dealing_phase_in_seconds: entity.dealing_phase_in_seconds,
            finishing_phase_in_seconds: entity.finishing_phase_in_seconds,
            number_of_cards_to_be_passed: entity.number_of_cards_to_be_passed,
            card_points: entity.card_points.iter().map(|kv| CardPointsDto { card: kv.0.into(), points: kv.1.clone() }).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardPointsDto {
    #[serde(rename="Card")]
    pub card: CardDto,
    #[serde(rename="Point")]
    pub points: i32,
}

impl TryFrom<CardPointsDto> for (Card, i32) {
    type Err = Error;

    fn try_from(dto: CardPointsDto) -> Result<(Card, i32)> {
        Ok((Card::try_from(dto.card)?, dto.points))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameParticipantDto {
    #[serde(rename="TeamName")]
    team_name: PlayerName,
    #[serde(rename="LeftParticipant")]
    left_participant: PlayerName,
    #[serde(rename="NumberOfCardsInHand")]
    number_of_cards_in_hand: u32,
    #[serde(rename="HasTurn")]
    has_turn: bool,
    #[serde(rename="CurrentScore")]
    current_score: i32,
}

impl From<GameParticipantDto> for GameParticipant {
    fn from(dto: GameParticipantDto) -> GameParticipant {
        GameParticipant {
            team_name: dto.team_name,
            left_participant: dto.left_participant,
            number_of_cards_in_hand: dto.number_of_cards_in_hand,
            has_turn: dto.has_turn,
            current_score: dto.current_score,
        }
    }
}

impl <'a> From<&'a GameParticipant> for GameParticipantDto {
    fn from(entity: &'a GameParticipant) -> GameParticipantDto {
        GameParticipantDto {
            team_name: entity.team_name.clone(),
            left_participant: entity.left_participant.clone(),
            number_of_cards_in_hand: entity.number_of_cards_in_hand,
            has_turn: entity.has_turn,
            current_score: entity.current_score,
        }
    }
}