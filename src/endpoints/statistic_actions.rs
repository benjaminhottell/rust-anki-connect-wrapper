use std::borrow::Cow;
use crate::endpoints::request::Request;
use crate::models::card_review::{
    CardReviewTuple,
    CardReviewWithoutCardId,
};

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

/// Corresponds to the `getCollectionStatsHTML` action
#[derive(serde::Serialize)]
pub struct GetCollectionStatsHtml {
    #[serde(rename = "wholeCollection")]
    whole_collection: Option<bool>,
}

impl GetCollectionStatsHtml {
    pub fn new(whole_collection: impl Into<Option<bool>>) -> Self {
        Self {
            whole_collection: whole_collection.into(),
        }
    }
}

impl Default for GetCollectionStatsHtml {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Request for GetCollectionStatsHtml {
    type Params = Self;
    type Response = String;
    fn get_action(&self) -> &'static str { "getCollectionStatsHTML" }
    fn get_params(&self) -> Option<&Self::Params> {
        match self.whole_collection {
            Some(_) => Some(self),
            _ => None,
        }
    }
}

/// Corresponds to the `cardReviews` action
#[derive(serde::Serialize)]
pub struct CardReviews<'a> {
    deck: Cow<'a, str>,
    #[serde(rename = "startID")]
    start_id: u64,
}

impl<'a> CardReviews<'a> {
    pub fn new(
        deck: impl Into<Cow<'a, str>>,
        start_id: u64,
    ) -> Self {
        Self {
            deck: deck.into(),
            start_id,
        }
    }
}

impl<'a> Request for CardReviews<'a> {
    type Response = Vec<CardReviewTuple>;
    type Params = Self;
    fn get_action(&self) -> &'static str { "cardReviews" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

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
    type Response = std::collections::HashMap<u64, Vec<CardReviewWithoutCardId>>;
    type Params = Self;
    fn get_action(&self) -> &'static str { "getReviewsOfCards" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `getLatestReviewID` action
#[derive(serde::Serialize)]
pub struct GetLatestReviewId<'a> {
    deck: Cow<'a, str>,
}

impl<'a> GetLatestReviewId<'a> {
    pub fn new(deck: impl Into<Cow<'a, str>>) -> Self {
        Self {
            deck: deck.into(),
        }
    }
}

impl<'a> Request for GetLatestReviewId<'a> {
    type Params = Self;
    type Response = u64;
    fn get_action(&self) -> &'static str { "getLatestReviewID" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

#[derive(serde::Serialize)]
pub struct InsertReviews<'a> {
    reviews: Cow<'a, [CardReviewTuple]>,
}

impl<'a> InsertReviews<'a> {

    pub fn new(reviews: Cow<'a, [CardReviewTuple]>) -> Self {
        InsertReviews {
            reviews,
        }
    }

    pub fn new_from<'b, T>(reviews: &'b [T]) -> Self where CardReviewTuple: std::convert::From<&'b T> {
        let reviews: Vec<CardReviewTuple> = reviews
            .iter()
            .map(|x| x.into())
            .collect::<Vec<CardReviewTuple>>();
        InsertReviews {
            reviews: reviews.into(),
        }
    }

}

impl<'a> Default for InsertReviews<'a> {
    fn default() -> Self {
        Self::new_from(&[])
    }
}
