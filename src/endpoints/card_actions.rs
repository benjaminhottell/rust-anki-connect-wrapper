use crate::client::Client;
use crate::error::Error;
use crate::models::RequestBody;


impl Client {

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

    /// Invokes the `findCards` action.
    /// Returns card IDs for cards that match the given query.
    /// See: <https://docs.ankiweb.net/searching.html>
    pub async fn find_cards(&self, query: &str) -> Result<Vec<u64>, Error> {
        let params = serde_json::json! {{
            "query": query,
        }};
        self.invoke(RequestBody::with_params("findCards", &params)).await
    }

    /// Invokes the `cardsToNotes` action.
    /// Returns all note IDs corresponding to the given card IDs.
    /// The order of elements in the output array is unspecified.
    pub async fn get_notes_from_cards(&self, cards: &Vec<u64>) -> Result<Vec<u64>, Error> {
        let params = serde_json::json! {{
            "cards": cards,
        }};
        self.invoke(RequestBody::with_params("cardsToNotes", &params)).await
    }

    // TODO cardsModTime
    // TODO cardsInfo
    // TODO forgetCards
    // TODO relearnCards
    // TODO answerCards
    // TODO setDueDate
}
