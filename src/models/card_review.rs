/// Represents a single review, or revlog entry.
/// Field names are based on the documentation for the `insertReviews` endpoint, which may differ
/// from how the fields are named in Anki's database or in Anki's backend code.
/// The fields in this struct should correspond to the fields in `RevlogEntry`.
/// https://github.com/ankitects/anki/blob/main/rslib/src/revlog/mod.rs
pub struct CardReview {

    /// The review's ID is also the time it occurred in milliseconds from UNIX epoch
    /// Backend type: `RevlogId`
    pub id: i64,

    /// Equivalent to `cid` in the database.
    /// Backend type: `CardId`
    pub card_id: i64,

    /// Backend type: `Usn`
    pub usn: i32,

    pub ease: u8,

    /// Aka. `ivl`
    pub new_interval: i32,

    /// Aka. `lastIvl`, `previous_interval`
    pub last_interval: i32,

    pub new_factor: u32,

    pub review_duration: u32,

    /// Aka. `review_kind`, `type`
    /// Backend type: `RevlogReviewKind`, backed by `u8`
    pub review_type: u8,
}

#[derive(serde::Deserialize)]
pub struct CardReviewWithoutCardId {

    pub id: i64,

    pub usn: i32,

    pub ease: u8,

    #[serde(rename = "ivl")]
    pub new_interval: i32,

    #[serde(rename = "lastIvl")]
    pub last_interval: i32,

    #[serde(rename = "factor")]
    pub new_factor: u32,

    #[serde(rename = "time")]
    pub review_duration: u32,

    #[serde(rename = "type")]
    pub review_type: u8,

}

impl CardReviewWithoutCardId {
    pub fn with_card_id(&self, card_id: i64) -> CardReview {
        CardReview {
            id: self.id,
            card_id,
            usn: self.usn,
            ease: self.ease,
            new_interval: self.new_interval,
            last_interval: self.last_interval,
            new_factor: self.new_factor,
            review_duration: self.review_duration,
            review_type: self.review_type,
        }
    }
}

impl From<CardReview> for CardReviewWithoutCardId {
    fn from(value: CardReview) -> Self {
        CardReviewWithoutCardId {
            id: value.id,
            usn: value.usn,
            ease: value.ease,
            new_interval: value.new_interval,
            last_interval: value.last_interval,
            new_factor: value.new_factor,
            review_duration: value.review_duration,
            review_type: value.review_type,
        }
    }
}

pub type CardReviewTuple = (i64, i64, i32, u8, i32, i32, u32, u32, u8);

impl CardReview {
    pub fn as_tuple(&self) -> CardReviewTuple {
        (
            self.id,
            self.card_id,
            self.usn,
            self.ease,
            self.new_interval,
            self.last_interval,
            self.new_factor,
            self.review_duration,
            self.review_type,
        )
    }
    pub fn from_tuple(value: &CardReviewTuple) -> Self {
        Self {
            id: value.0,
            card_id: value.1,
            usn: value.2,
            ease: value.3,
            new_interval: value.4,
            last_interval: value.5,
            new_factor: value.6,
            review_duration: value.7,
            review_type: value.8,
        }
    }
}

impl From<&CardReviewTuple> for CardReview {
    fn from(value: &CardReviewTuple) -> Self {
        CardReview::from_tuple(value)
    }
}

impl From<&CardReview> for CardReviewTuple {
    fn from(value: &CardReview) -> Self {
        value.as_tuple()
    }
}
