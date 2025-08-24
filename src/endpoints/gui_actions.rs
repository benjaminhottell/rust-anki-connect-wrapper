use std::borrow::Cow;

use crate::endpoints::request::Request;
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
pub struct GuiBrowseOptions<'a> {

    #[serde(
        skip_serializing_if = "Option::is_none"
    )]
    pub query: Option<Cow<'a, str>>,

    #[serde(
        rename = "reorderCards",
        skip_serializing_if = "Option::is_none"
    )]
    pub reorder_cards: Option<GuiBrowseCardOrderOptions>,
}

impl<'a> GuiBrowseOptions<'a> {

    pub fn new() -> Self {
        Self {
            query: None,
            reorder_cards: None,
        }
    }

    /// Create an options object with just a query and without any sort preferences
    pub fn query(query: impl Into<Cow<'a, str>>) -> Self {
        Self {
            query: Some(query.into()),
            reorder_cards: None,
        }
    }
}

impl<'a> Default for GuiBrowseOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> GuiBrowseOptions<'a> {

    pub fn with_query(mut self, query: Cow<'a, str>) -> Self {
        self.query = Some(query);
        self
    }

}

/// Corresponds to the `guiBrowse` action.
/// Returns card IDs that match the given query.
/// See: <https://docs.ankiweb.net/searching.html>
pub struct GuiBrowse<'a> {
    options: Option<GuiBrowseOptions<'a>>,
}

impl<'a> GuiBrowse<'a> {

    pub fn new(options: Option<GuiBrowseOptions<'a>>) -> Self {
        Self {
            options,
        }
    }

    pub fn builder() -> GuiBrowseBuilder<'a> {
        GuiBrowseBuilder::new()
    }

}

impl<'a> Default for GuiBrowse<'a> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<'a> Request for GuiBrowse<'a> {
    type Params = GuiBrowseOptions<'a>;
    type Response = Vec<u64>;
    fn get_action(&self) -> &'static str { "guiBrowse" }
    fn get_params(&self) -> Option<&Self::Params> { self.options.as_ref() }
}

pub struct GuiBrowseBuilder<'a> {
    order: Option<Order>,
    column: Option<BrowserColumn>,
    query: Option<Cow<'a, str>>,
}

impl<'a> GuiBrowseBuilder<'a> {

    pub fn new() -> Self {
        Self {
            order: None,
            column: None,
            query: None,
        }
    }

    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    pub fn ascending(self) -> Self {
        self.order(Order::Ascending)
    }

    pub fn descending(self) -> Self {
        self.order(Order::Descending)
    }

    pub fn column(mut self, column: BrowserColumn) -> Self {
        self.column = Some(column);
        self
    }

    pub fn query(mut self, query: impl Into<Cow<'a, str>>) -> Self {
        self.query = Some(query.into());
        self
    }

    fn build_options(self) -> Option<GuiBrowseOptions<'a>> {
        let order = self.order;
        let column = self.column;
        let query = self.query;

        let reorder_cards = {
            if order.is_none() && column.is_none() {
                None
            } else {
                Some(GuiBrowseCardOrderOptions {
                    order,
                    column,
                })
            }
        };

        if query.is_none() && reorder_cards.is_none() {
            return None;
        }

        Some(GuiBrowseOptions {
            query,
            reorder_cards,
        })
    }

    pub fn build(self) -> GuiBrowse<'a> {
        GuiBrowse::new(self.build_options())
    }

}

impl<'a> Default for GuiBrowseBuilder<'a> {
    fn default() -> Self {
        GuiBrowseBuilder::new()
    }
}

/// Corresponds to the `guiSelectCard` action.
/// Expects the Card Browser dialog to already be open.
/// Returns `true` if the Card Browser dialog is open.
#[derive(serde::Serialize)]
pub struct GuiSelectCard {
    /// Card ID
    card: u64,
}

impl Request for GuiSelectCard {
    type Params = Self;
    type Response = bool;
    fn get_action(&self) -> &'static str { "guiSelectCard" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO guiSelectedNotes
// TODO guiAddCards
// TODO guiEditNote
// TODO guiCurrentCard
// TODO guiStartCardTimer
// TODO guiShowQuestion
// TODO guiShowAnswer
// TODO guiAnswerCard

/// Corresponds to the `guiUndo` action.
/// Returns `true` on success.
pub struct GuiUndo;

impl Request for GuiUndo {
    type Params = ();
    type Response = bool;
    fn get_action(&self) -> &'static str { "guiUndo" }
}

/// Invokes the `guiDeckOverview` action
/// Opens the Deck Overview dialog for the given deck.
#[derive(serde::Serialize)]
pub struct GuiDeckOverview<'a> {
    /// Name of deck
    name: Cow<'a, str>,
}

impl<'a> Request for GuiDeckOverview<'a> {
    type Params = Self;
    type Response = bool;
    fn get_action(&self) -> &'static str { "guiDeckOverview" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Invokes the `guiDeckBrowser` action.
/// Opens the Deck Browser dialog.
pub struct GuiDeckBrowser;

impl Request for GuiDeckBrowser {
    type Params = ();
    type Response = ();
    fn get_action(&self) -> &'static str { "guiDeckBrowser" }
}

/// Invokes the `guiDeckReview` action
/// Starts reviewing the deck. Returns `true` on success.
#[derive(serde::Serialize)]
pub struct GuiDeckReview<'a> {
    /// Name of deck
    name: Cow<'a, str>,
}

impl<'a> Request for GuiDeckReview<'a> {
    type Params = Self;
    type Response = bool;
    fn get_action(&self) -> &'static str { "guiDeckReview" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Invokes the `guiImportFile` action.
/// Opens the Import... dialog.
#[derive(serde::Serialize)]
pub struct GuiImportFile<'a> {
    path: Option<Cow<'a, str>>,
}

impl<'a> Request for GuiImportFile<'a> {
    type Params = Self;
    type Response = ();
    fn get_action(&self) -> &'static str { "guiImportFile" }
    fn get_params(&self) -> Option<&Self::Params> {
        match self.path {
            Some(_) => Some(self),
            _ => None,
        }
    }
}

/// Invokes the `guiExitAnki` action.
/// According to API docs, the API request will return immediately and does not wait for Anki to
/// actually close.
pub struct GuiExitAnki;

impl Request for GuiExitAnki {
    type Params = ();
    type Response = ();
    fn get_action(&self) -> &'static str { "guiExitAnki" }
}

/// Invokes the `guiCheckDatabase` action.
/// According to API docs, it should always return `true`.
pub struct GuiCheckDatabase;

impl Request for GuiCheckDatabase {
    type Params = ();
    type Response = ();
    fn get_action(&self) -> &'static str { "guiCheckDatabase" }
}
