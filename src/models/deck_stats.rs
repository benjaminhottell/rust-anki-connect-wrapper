#[derive(serde::Deserialize)]
pub struct DeckStats {
    pub deck_id: u64,
    pub name: String,
    pub new_count: u64,
    pub learn_count: u64,
    pub review_count: u64,
    pub total_in_deck: u64,
}
