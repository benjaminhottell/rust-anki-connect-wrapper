/// Represents data returned by the `getReviewsOfCards` action.
/// A few fields are renamed to better adhere to Rust conventions and avoid keywords.
#[derive(serde::Deserialize)]
pub struct CardReview {
    /// The review's ID is also the time it occurred in milliseconds from UNIX epoch
    pub id: u64,
    pub usn: u64,
    pub ease: u64,
    pub ivl: i64,
    #[serde(rename(deserialize = "lastIvl", serialize = "lastIvl"))]
    pub last_ivl: i64,
    pub factor: u64,
    pub time: u64,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub review_type: u64,
}
