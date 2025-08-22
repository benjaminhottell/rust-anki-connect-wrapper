use crate::models::CardReview;
use crate::endpoints::request::Request;

// Statistic actions

// TODO getNumCardsReviewedByDay
// TODO getCollectionStatsHTML
// TODO cardReviews

/// Corresponds to the `getNumCardsReviewedToday` action.
pub struct GetNumCardsReviewedToday {}

impl Request for GetNumCardsReviewedToday {
    type Response = u64;
    type Params = ();
    fn get_action(&self) -> &'static str { "getNumCardsReviewedToday" }
    fn get_params(&self) -> Option<&Self::Params> { None }
}

/// Corresponds to the `getReviewsOfCards` action.
#[derive(serde::Serialize)]
pub struct GetReviewsOfCards<'a> {
    /// Card IDs
    cards: &'a [u64],
}

impl<'a> GetReviewsOfCards<'a> {
    pub fn new(cards: &'a [u64]) -> GetReviewsOfCards<'a> {
        GetReviewsOfCards {
            cards,
        }
    }
}

impl<'a> Request for GetReviewsOfCards<'a> {
    type Response = std::collections::HashMap<u64, Vec<CardReview>>;
    type Params = Self;
    fn get_action(&self) -> &'static str { "getReviewsOfCards" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO getLatestReviewID
// TODO insertReviews
