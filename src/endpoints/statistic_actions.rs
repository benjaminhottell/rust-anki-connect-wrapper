use crate::client::Client;
use crate::error::Error;
use crate::models::{RequestBody, CardReview};

impl Client {

    // Statistic actions

    /// Invokes the `getNumCardsReviewedToday` action.
    pub async fn get_num_cards_reviewed_today(&self) -> Result<u64, Error> {
        self.invoke(RequestBody::without_params("getNumCardsReviewedToday")).await
    }

    // TODO getNumCardsReviewedByDay
    // TODO getCollectionStatsHTML
    // TODO cardReviews

    /// Invokes the `getReviewsOfCards` action.
    /// Returns a mapping of card ID to associated reviews.
    pub async fn get_reviews_of_cards(
        &self,
        cards: &[u64],
    ) -> Result<std::collections::HashMap<u64, Vec<CardReview>>, Error> {
        let params = serde_json::json! {{
            "cards": cards,
        }};
        self.invoke(RequestBody::with_params("getReviewsOfCards", &params)).await
    }

    // TODO getLatestReviewID
    // TODO insertReviews
}
