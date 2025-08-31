use std::borrow::Cow;
use crate::endpoints::request::Request;

// Card actions

/// Correponds to the `getEaseFactors` action.
#[derive(serde::Serialize)]
pub struct GetEaseFactors<'a> {
    cards: &'a [u64],
}

impl<'a> GetEaseFactors<'a> {
    pub fn new(cards: &'a [u64]) -> Self {
        Self {
            cards,
        }
    }
}

impl<'a> Request for GetEaseFactors<'a> {
    type Params = Self;
    type Response = Vec<u64>;
    fn get_action(&self) -> &'static str { "getEaseFactors" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

impl<'a> Default for GetEaseFactors<'a> {
    fn default() -> Self {
        GetEaseFactors::new(&[])
    }
}

// TODO setEaseFactors
// TODO setSpecificValueOfCard
// TODO suspend
// TODO unsuspend
// TODO suspended
// TODO areSuspended

/// Corresponds to the `areDue` action.
#[derive(serde::Serialize)]
pub struct AreDue<'a> {
    cards: Cow<'a, [u64]>,
}

impl<'a> AreDue<'a> {
    pub fn new(cards: impl Into<Cow<'a, [u64]>>) -> Self {
        Self {
            cards: cards.into(),
        }
    }
}

impl<'a> Default for AreDue<'a> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a> Request for AreDue<'a> {
    type Params = Self;
    type Response = Vec<bool>;
    fn get_action(&self) -> &'static str { "areDue" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO getIntervals

/// Corresponds to the `findCards` action.
/// Returns card IDs for cards that match the given query.
/// See: <https://docs.ankiweb.net/searching.html>
#[derive(serde::Serialize)]
pub struct FindCards<'a> {
    query: Cow<'a, str>,
}

impl<'a> FindCards<'a> {
    pub fn new(query: impl Into<Cow<'a, str>>) -> FindCards<'a> {
        FindCards {
            query: query.into(),
        }
    }
}

impl<'a> Request for FindCards<'a> {
    type Params = Self;
    type Response = Vec<u64>;
    fn get_action(&self) -> &'static str { "findCards" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

impl<'a> Default for FindCards<'a> {
    fn default() -> Self {
        FindCards::new("")
    }
}

/// Invokes the `cardsToNotes` action.
/// Returns all note IDs corresponding to the given card IDs.
/// The order of elements in the output array is unspecified.
#[derive(serde::Serialize)]
pub struct CardsToNotes<'a> {
    cards: Cow<'a, [u64]>,
}

impl<'a> CardsToNotes<'a> {
    pub fn new(cards: impl Into<Cow<'a, [u64]>>) -> Self {
        Self {
            cards: cards.into(),
        }
    }
}

impl<'a> Request for CardsToNotes<'a> {
    type Params = Self;
    type Response = Vec<u64>;
    fn get_action(&self) -> &'static str { "cardsToNotes" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO cardsModTime
// TODO cardsInfo
// TODO forgetCards
// TODO relearnCards
// TODO answerCards

/// Corresponds to the `setDueDate` action
#[derive(serde::Serialize)]
pub struct SetDueDate<'a> {
    cards: Cow<'a, [u64]>,
    days: Cow<'a, str>,
}

impl<'a> SetDueDate<'a> {
    pub fn new(
        cards: impl Into<Cow<'a, [u64]>>,
        days: impl Into<Cow<'a, str>>,
    ) -> Self {
        SetDueDate {
            cards: cards.into(),
            days: days.into(),
        }
    }
}

impl<'a> Request for SetDueDate<'a> {
    type Params = Self;
    type Response = ();
    fn get_action(&self) -> &'static str { "setDueDate" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}
