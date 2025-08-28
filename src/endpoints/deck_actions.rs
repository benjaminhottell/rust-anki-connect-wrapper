use std::borrow::Cow;
use crate::endpoints::request::Request;
use crate::models::DeckStats;

/// Corresponds to the `deckNames` action
#[derive(Default)]
pub struct DeckNames;

impl Request for DeckNames {
    type Params = ();
    type Response = Vec<String>;
    fn get_action(&self) -> &'static str { "deckNames" }
}

/// Corresponds to the `deckNamesAndIds` action
#[derive(Default)]
pub struct DeckNamesAndIds;

impl Request for DeckNamesAndIds {
    type Params = ();
    type Response = std::collections::HashMap<String, u64>;
    fn get_action(&self) -> &'static str { "deckNamesAndIds" }
}

/// Corresponds to the `getDecks` action.
#[derive(serde::Serialize)]
pub struct GetDecks<'a> {
    cards: Cow<'a, [u64]>,
}

impl<'a> GetDecks<'a> {
    fn new(cards: impl Into<Cow<'a, [u64]>>) -> Self {
        Self {
            cards: cards.into(),
        }
    }
}

impl<'a> Default for GetDecks<'a> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a> Request for GetDecks<'a> {
    type Params = Self;
    type Response = std::collections::HashMap<String, Vec<u64>>;
    fn get_action(&self) -> &'static str { "getDecks" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `createDeck` action.
#[derive(serde::Serialize)]
pub struct CreateDeck<'a> {
    /// Name of deck
    deck: Cow<'a, str>,
}

impl<'a> CreateDeck<'a> {
    pub fn new(deck: impl Into<Cow<'a, str>>) -> Self {
        Self {
            deck: deck.into(),
        }
    }
}

impl<'a> Request for CreateDeck<'a> {
    type Params = Self;
    type Response = u64;
    fn get_action(&self) -> &'static str { "createDeck" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `changeDeck` action
#[derive(serde::Serialize)]
pub struct ChangeDeck<'a> {
    /// Card IDs to move
    cards: Cow<'a, [u64]>,
    /// Name of deck to move them to
    deck: Cow<'a, str>,
}

impl<'a> ChangeDeck<'a> {
    pub fn new(
        cards: impl Into<Cow<'a, [u64]>>,
        deck: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            cards: cards.into(),
            deck: deck.into(),
        }
    }
}

impl<'a> Request for ChangeDeck<'a> {
    type Params = Self;
    type Response = ();
    fn get_action(&self) -> &'static str { "changeDeck" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `deleteDecks` action
#[derive(serde::Serialize)]
pub struct DeleteDecks<'a> {
    /// Names of decks
    decks: Vec<Cow<'a, str>>
}

impl<'a> DeleteDecks<'a> {
    pub fn new(decks: impl Into<Vec<Cow<'a, str>>>) -> Self {
        Self {
            decks: decks.into(),
        }
    }
    pub fn single(deck: impl Into<Cow<'a, str>>) -> Self {
        Self::new([deck.into()])
    }
}

impl<'a> Request for DeleteDecks<'a> {
    type Params = Self;
    type Response = ();
    fn get_action(&self) -> &'static str { "deleteDecks" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO getDeckConfig
// TODO saveDeckConfig
// TODO cloneDeckConfigId
// TODO removeDeckConfigId

/// Corresponds to the `getDeckStats` action
#[derive(serde::Serialize)]
pub struct GetDeckStats<'a> {
    /// Deck names
    decks: Vec<Cow<'a, str>>
}

impl<'a> GetDeckStats<'a> {
    pub fn new(decks: impl Into<Vec<Cow<'a, str>>>) -> Self {
        Self {
            decks: decks.into(),
        }
    }
    pub fn single(deck: impl Into<Cow<'a, str>>) -> Self {
        Self::new([deck.into()])
    }
}

impl<'a> Default for GetDeckStats<'a> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a> Request for GetDeckStats<'a> {
    type Params = Self;
    type Response = std::collections::HashMap<u64, DeckStats>;
    fn get_action(&self) -> &'static str { "getDeckStats" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}
