use crate::models::{Order, BrowserColumn};

#[derive(serde::Serialize)]
pub struct GuiBrowseCardOrderOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,

    #[serde(
        rename = "columnId",
        skip_serializing_if = "Option::is_none"
    )]
    pub column: Option<BrowserColumn>,
}

impl GuiBrowseCardOrderOptions {
    pub fn new() -> Self {
        Self {
            order: None,
            column: None,
        }
    }
}

impl Default for GuiBrowseCardOrderOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(serde::Serialize)]
pub struct GuiBrowseOptions {

    #[serde(
        skip_serializing_if = "Option::is_none"
    )]
    pub query: Option<String>,

    #[serde(
        rename = "reorderCards",
        skip_serializing_if = "Option::is_none"
    )]
    pub reorder_cards: Option<GuiBrowseCardOrderOptions>,
}

impl GuiBrowseOptions {

    pub fn new() -> Self {
        Self {
            query: None,
            reorder_cards: None,
        }
    }

    /// Create an options object with just a query and without any sort preferences
    pub fn query(query: impl Into<String>) -> Self {
        Self {
            query: Some(query.into()),
            reorder_cards: None,
        }
    }
}

impl Default for GuiBrowseOptions {
    fn default() -> Self {
        Self::new()
    }
}
