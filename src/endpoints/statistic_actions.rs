use std::borrow::Cow;
use crate::models::CardReview;
use crate::endpoints::request::Request;

// Statistic actions

/// Corresponds to the `getNumCardsReviewedToday` action.
#[derive(Default)]
pub struct GetNumCardsReviewedToday;

impl Request for GetNumCardsReviewedToday {
    type Response = u64;
    type Params = ();
    fn get_action(&self) -> &'static str { "getNumCardsReviewedToday" }
    fn get_params(&self) -> Option<&Self::Params> { None }
}

/// Corresponds to the `getNumCardsReviewedByDay` action
#[derive(Default)]
pub struct GetNumCardsReviewedByDay;

impl Request for GetNumCardsReviewedByDay {
    type Response = Vec<(String, u64)>;
    type Params = ();
    fn get_action(&self) -> &'static str { "getNumCardsReviewedByDay" }
    fn get_params(&self) -> Option<&Self::Params> { None }
}

// TODO getCollectionStatsHTML
// TODO cardReviews

/// Corresponds to the `getReviewsOfCards` action.
#[derive(serde::Serialize)]
pub struct GetReviewsOfCards<'a> {
    /// Card IDs
    cards: Cow<'a, [u64]>,
}

impl<'a> GetReviewsOfCards<'a> {
    pub fn new(cards: impl Into<Cow<'a, [u64]>>) -> GetReviewsOfCards<'a> {
        GetReviewsOfCards {
            cards: cards.into(),
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
