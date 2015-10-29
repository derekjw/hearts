use game_status::RoundParameters;
use game_status::GameStatus;
use game_status::GameInstanceState;
use game_status::RoundState;
use game_status::HeartsGameInstanceState;

use card::Card;
use card::dto::CardDto;
use deal::Deal;
use deal::dto::DealDto;
use player::PlayerName;
use try_from::TryFrom;
use try_from::TryInto;
use error::Error;
use error::Result;

use std::str::FromStr;

#[derive(Deserialize, Debug)]
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
            Some(result) => result.map(|value| Some(value)),
            None => Ok(None)
        };
        Ok(GameStatus {
            current_game_id: dto.current_game_id,
            current_game_state: try!(GameInstanceState::from_str(&dto.current_game_state)),
            current_round_id: dto.current_round_id,
            current_round_state: try!(RoundState::from_str(&dto.current_round_state)),
            round_parameters: try!(RoundParameters::try_from(dto.round_parameters)),
            my_game_state: try!(HeartsGameInstanceState::from_str(&dto.my_game_state)),
            my_game_players: dto.my_game_participants.into_iter().map(|participant| participant.team_name).collect(),
            my_initial_hand: try!(dto.my_initial_hand.into_iter().map(Card::try_from).collect()),
            my_final_hand: try!(dto.my_final_hand.into_iter().map(Card::try_from).collect()),
            my_current_hand: try!(dto.my_current_hand.into_iter().map(Card::try_from).collect()),
            my_game_deals: try!(dto.my_game_deals.into_iter().map(Deal::try_from).collect()),
            my_in_progress_deal: try!(my_in_progress_deal),
            is_my_turn: dto.is_my_turn,
        })
    }
}

#[derive(Deserialize, Debug, Default)]
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
            card_points: try!(dto.card_points.into_iter().map(CardPointsDto::try_into).collect()),
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct CardPointsDto {
    #[serde(rename="Card")]
    card: CardDto,
    #[serde(rename="Point")]
    points: i32,
}

impl TryFrom<CardPointsDto> for (Card, i32) {
    type Err = Error;

    fn try_from(dto: CardPointsDto) -> Result<(Card, i32)> {
        Ok((try!(Card::try_from(dto.card)), dto.points))
    }
}

#[derive(Deserialize, Debug)]
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
