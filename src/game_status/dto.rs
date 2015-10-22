use card::dto::CardDto;

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
    #[serde(rename="RoundParameters")]
    round_parameters: RoundParametersDto,
    #[serde(rename="MyGameState")]
    my_game_state: String,
    #[serde(rename="MyGameStateDescription")]
    my_game_state_description: String,
    #[serde(rename="MyGameParticipants")]
    my_game_participants: Vec<GameParticipantDto>,
    #[serde(rename="MyInitialHand")]
    my_initial_hand: Vec<CardDto>,
    #[serde(rename="CardsPassedByMe")]
    cards_passed_by_me: Vec<CardDto>,
    #[serde(rename="CardsPassedToMe")]
    cards_passed_to_me: Vec<CardDto>,
    #[serde(rename="MyFinalHand")]
    my_final_hand: Vec<CardDto>,
    #[serde(rename="MyCurrentHand")]
    my_current_hand: Vec<CardDto>,
    #[serde(rename="MyGameDeals")]
    my_game_deals: Vec<DealDto>,
    #[serde(rename="MyInProgressDeal")]
    my_in_progress_deal: InProgressDealDto,
    #[serde(rename="IsMyTurn")]
    is_my_turn: bool,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct CardPointsDto {
    #[serde(rename="Card")]
    card: CardDto,
    #[serde(rename="Point")]
    points: u32,
}

#[derive(Deserialize, Debug)]
pub struct GameParticipantDto {
    #[serde(rename="TeamName")]
    team_name: String,
    #[serde(rename="LeftParticipant")]
    left_participant: String,
    #[serde(rename="NumberOfCardsInHand")]
    number_of_cards_in_hand: u32,
    #[serde(rename="HasTurn")]
    has_turn: bool,
    #[serde(rename="CurrentScore")]
    current_score: u32,
}

#[derive(Deserialize, Debug)]
pub struct DealDto {
    #[serde(rename="DealNumber")]
    deal_number: u32,
    #[serde(rename="Initiator")]
    initiator: String,
    #[serde(rename="SuitType")]
    suit_type: String,
    #[serde(rename="DealCards")]
    deal_cards: Vec<DealCardDto>,
    #[serde(rename="DealWinner")]
    deal_winner: String,
}


#[derive(Deserialize, Debug)]
pub struct InProgressDealDto {
    #[serde(rename="DealNumber")]
    deal_number: u32,
    #[serde(rename="Initiator")]
    initiator: String,
    #[serde(rename="SuitType")]
    suit_type: String,
    #[serde(rename="DealCards")]
    deal_cards: Vec<DealCardDto>,
}

#[derive(Deserialize, Debug)]
pub struct DealCardDto {
    #[serde(rename="TeamName")]
    team_name: String,
    #[serde(rename="Card")]
    card: CardDto,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
    }

}
