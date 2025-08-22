use crate::endpoints::request::Request;

// Card actions

// TODO getEaseFactors
// TODO setEaseFactors
// TODO setSpecificValueOfCard
// TODO suspend
// TODO unsuspend
// TODO suspended
// TODO areSuspended
// TODO areDue
// TODO getIntervals

/// Corresponds to the `findCards` action.
/// Returns card IDs for cards that match the given query.
/// See: <https://docs.ankiweb.net/searching.html>
#[derive(serde::Serialize)]
pub struct FindCards<'a> {
    query: &'a str,
}

impl<'a> FindCards<'a> {
    pub fn new(query: &'a str) -> FindCards<'a> {
        FindCards {
            query,
        }
    }
}

impl<'a> Request for FindCards<'a> {
    type Params = Self;
    type Response = Vec<u64>;
    fn get_action(&self) -> &'static str { "findCards" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Invokes the `cardsToNotes` action.
/// Returns all note IDs corresponding to the given card IDs.
/// The order of elements in the output array is unspecified.
#[derive(serde::Serialize)]
pub struct CardsToNotes<'a> {
    cards: &'a Vec<u64>,
}

impl<'a> CardsToNotes<'a> {
    pub fn new(cards: &'a Vec<u64>) -> CardsToNotes<'a> {
        CardsToNotes {
            cards,
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
// TODO setDueDate
