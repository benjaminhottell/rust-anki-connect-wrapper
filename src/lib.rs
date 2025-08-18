// Relevant docs: https://git.sr.ht/~foosoft/anki-connect

use strum::{EnumString, Display};

/// The full URL that Anki-Connect runs on by default
pub const DEFAULT_URL: &str = "http://127.0.0.1:8765";

#[derive(EnumString, Display)]
#[strum(serialize_all = "camelCase")]
pub enum Order {
    Ascending,
    Descending,
}

impl From<Order> for serde_json::Value {
    fn from(value: Order) -> Self {
        serde_json::Value::String(value.to_string())
    }
}

/// I mimic Anki's original enum members and serialization rules as closely as possible.
/// See: https://github.com/ankitects/anki/blob/main/rslib/src/browser_table.rs
#[derive(EnumString, Display)]
#[strum(serialize_all = "camelCase")]
pub enum BrowserColumn {
    Answer,
    CardMod,
    #[strum(serialize = "template")]
    Cards,
    Deck,
    #[strum(serialize = "cardDue")]
    Due,
    #[strum(serialize = "cardEase")]
    Ease,
    #[strum(serialize = "cardLapses")]
    Lapses,
    #[strum(serialize = "cardIvl")]
    Interval,
    #[strum(serialize = "noteCrt")]
    NoteCreation,
    NoteMod,
    #[strum(serialize = "note")]
    Notetype,
    OriginalPosition,
    Question,
    #[strum(serialize = "cardReps")]
    Reps,
    #[strum(serialize = "noteFld")]
    SortField,
    #[strum(serialize = "noteTags")]
    Tags,
    Stability,
    Difficulty,
    Retrievability,
}

impl From<BrowserColumn> for serde_json::Value {
    fn from(value: BrowserColumn) -> Self {
        serde_json::Value::String(value.to_string())
    }
}

/// Represents data returned by the `getReviewsOfCards` action.
/// A few fields are renamed to better adhere to Rust conventions and avoid keywords.
#[derive(Debug, serde::Deserialize)]
pub struct ReviewOfCard {
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

#[derive(Debug)]
pub enum DeserializeError {
    Serde(serde_json::Error),
    OtherStatic(&'static str),
}

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeserializeError::Serde(e) => {
                write!(f, "{e}")
            }
            DeserializeError::OtherStatic(e) => {
                write!(f, "{e}")
            }
        }
    }
}

/// Everything that can go wrong with any request (invocation) to the Anki-Connect API
#[derive(Debug)]
pub enum Error {
    /// Occurs if there was an error making a request to the API
    Request(reqwest::Error),

    /// Occurs if the API responds to the request but does not do so with valid JSON
    Parse(reqwest::Error),

    /// Occurs if there is an issue deserializing an otherwise syntactically valid JSON result.
    /// Should never appear if the API is working as expected.
    Deserialize(DeserializeError),

    /// When the request is otherwise OK and everything is valid, but the API operation did not
    /// complete successfully
    Normal(serde_json::Value),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Request(e) => {
                write!(f, "error making request: {e}")
            }
            Error::Parse(e) => {
                write!(f, "error parsing response as JSON: {e}")
            }
            Error::Deserialize(e) => {
                write!(f, "error deserializing response: {e}")
            }
            Error::Normal(x) => {
                write!(f, "got error response: {x}")
            }
        }
    }
}

pub struct AnkiConnectBuilder {
    url: Option<String>,
    client: Option<reqwest::Client>,
}

impl AnkiConnectBuilder {

    pub fn new() -> AnkiConnectBuilder {
        AnkiConnectBuilder {
            url: None,
            client: None,
        }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> AnkiConnect {
        AnkiConnect::new(
            self.url.unwrap_or_else(|| DEFAULT_URL.to_string()),
            self.client.unwrap_or_default(),
        )
    }

}

impl Default for AnkiConnectBuilder {
    fn default() -> Self {
        AnkiConnectBuilder::new()
    }
}

pub struct AnkiConnect {
    url: String,
    client: reqwest::Client,
}

impl AnkiConnect {

    pub fn builder() -> AnkiConnectBuilder {
        AnkiConnectBuilder::new()
    }

    fn new(url: String, client: reqwest::Client) -> AnkiConnect {
        AnkiConnect { url, client }
    }

    pub async fn invoke(
        &self,
        action: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value, Error> {

        let body = serde_json::json!({
            "action": &action,
            "version": 6,
            "params": &params,
        });

        let res = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(Error::Request)?;

        let body: serde_json::Value = res.json()
            .await
            .map_err(Error::Parse)?;

        let mut body = match body {
            serde_json::Value::Object(map) => map,
            _ => {
                return Err(Error::Deserialize(DeserializeError::OtherStatic(
                    "root element is not an object",
                )));
            }
        };

        if body.len() != 2 {
            return Err(Error::Deserialize(DeserializeError::OtherStatic(
                "root object has an unexpected amount of fields",
            )));
        }

        let res_error =
            body.remove("error")
                .ok_or(Error::Deserialize(DeserializeError::OtherStatic(
                    "root object is missing \"error\" field",
                )))?;

        let res_result =
            body.remove("result")
                .ok_or(Error::Deserialize(DeserializeError::OtherStatic(
                    "root object is missing \"result\" field",
                )))?;

        if !res_error.is_null() {
            return Err(Error::Normal(res_error));
        }

        Ok(res_result)
    }

    pub async fn invoke_without_params(&self, action: &str) -> Result<serde_json::Value, Error> {
        let params = serde_json::json! {{}};
        return self.invoke(action, &params).await;
    }

    pub async fn invoke_de<T: serde::de::DeserializeOwned>(
        &self,
        action: &str,
        params: &serde_json::Value,
    ) -> Result<T, Error> {
        self.invoke(action, params)
            .await
            .map(serde_json::from_value::<T>)?
            .map_err(DeserializeError::Serde)
            .map_err(Error::Deserialize)
    }

    pub async fn invoke_de_without_params<T: serde::de::DeserializeOwned>(
        &self,
        action: &str,
    ) -> Result<T, Error> {
        let params = serde_json::json! {{}};
        return self.invoke_de(action, &params).await;
    }

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
        self.invoke_de("findCards", &params).await
    }

    /// Invokes the `cardsToNotes` action.
    /// Returns all note IDs corresponding to the given card IDs.
    /// The order of elements in the output array is unspecified.
    pub async fn get_notes_from_cards(&self, cards: &Vec<u64>) -> Result<Vec<u64>, Error> {
        let params = serde_json::json! {{
            "cards": cards,
        }};
        self.invoke_de("cardsToNotes", &params).await
    }

    // TODO cardsModTime
    // TODO cardsInfo
    // TODO forgetCards
    // TODO relearnCards
    // TODO answerCards
    // TODO setDueDate

    // Deck actions
    // TODO

    // Graphical actions

    /// Invokes the `guiBrowse` action.
    /// Returns card IDs that match the given query.
    /// See: <https://docs.ankiweb.net/searching.html>
    pub async fn do_gui_browse(
        &self,
        query: &str,
        order: Option<Order>,
        order_column: Option<BrowserColumn>,
    ) -> Result<Vec<u64>, Error> {

        let mut params = serde_json::Map::new();
        params.insert("query".to_string(), query.into());

        if order.is_some() || order_column.is_some() {
            let mut reorder_cards = serde_json::Map::new();
            if let Some(x) = order {
                reorder_cards.insert("order".to_string(), x.into());
            }
            if let Some(x) = order_column {
                reorder_cards.insert("columnId".to_string(), x.into());
            }
            params.insert("reorderCards".to_string(), serde_json::Value::Object(reorder_cards));
        }

        let params = serde_json::Value::Object(params);
        eprintln!("{params}");
        self.invoke_de("guiBrowse", &params).await
    }

    // TODO guiSelectCard
    // TODO guiSelectedNotes
    // TODO guiAddCards
    // TODO guiEditNote
    // TODO guiCurrentCard
    // TODO guiStartCardTimer
    // TODO guiShowQuestion
    // TODO guiShowAnswer
    // TODO guiAnswerCard
    // TODO guiUndo
    // TODO guiDeckOverview
    // TODO guiDeckBrowser
    // TODO guiDeckReview
    // TODO guiImportFile
    // TODO guiExitAnki
    // TODO guiCheckDatabase

    // Media actions
    // TODO

    // Miscellaneous Actions

    // TODO requestPermission

    /// Invokes the `version` action.
    /// Returns a number indicating the version of the API server.
    pub async fn get_version(&self) -> Result<u64, Error> {
        self.invoke_de_without_params("version").await
    }

    // TODO apiReflect

    /// Invokes the `sync` action.
    /// Prompts the client to sync with AnkiWeb.
    pub async fn sync(&self) -> Result<(), Error> {
        self.invoke_de_without_params("sync").await
    }

    /// Invokes the `getProfiles` action.
    /// Returns a Vec of strings indicating the names of profiles in the client.
    pub async fn get_profiles(&self) -> Result<Vec<String>, Error> {
        self.invoke_de_without_params("getProfiles").await
    }

    /// Invokes the `getActiveProfile` action.
    /// Returns a string indicating the name of the active profile.
    pub async fn get_active_profile(&self) -> Result<String, Error> {
        self.invoke_de_without_params("getActiveProfile").await
    }

    // TODO loadProfile
    // TODO multi
    // TODO exportPackage
    // TODO importPackage
    // TODO reloadCollection

    // Model actions
    // TODO

    // Note actions

    // TODO addNote
    // TODO addNotes
    // TODO canAddNotes
    // TODO canAddNotesWithErrorDetail
    // TODO updateNoteFields
    // TODO updateNote
    // TODO updateNoteModel
    // TODO updateNoteTags
    // TODO getNoteTags
    // TODO addTags

    /// Invokes the `addTags` action.
    /// `notes` should be a Vec of note IDs.
    /// `tags` should be the tags to add, separated by space.
    pub async fn add_tags(&self, notes: &Vec<u64>, tags: &str) -> Result<(), Error> {
        let params = serde_json::json! {{
            "notes": notes,
            "tags": tags,
        }};
        self.invoke_de("addTags", &params).await
    }

    /// Invokes the `removeTags` action.
    /// `notes` should be a Vec of note IDs.
    /// `tags` should be the tags to remove, separated by space.
    pub async fn remove_tags(&self, notes: &Vec<u64>, tags: &str) -> Result<(), Error> {
        let params = serde_json::json! {{
            "notes": notes,
            "tags": tags,
        }};
        self.invoke_de("removeTags", &params).await
    }

    // TODO getTags
    // TODO clearUnusedTags
    // TODO replaceTags
    // TODO replaceTagsInAllNotes

    /// Invokes the `findNotes` action.
    /// Returns note IDs for notes that match the given query.
    /// See: <https://docs.ankiweb.net/searching.html>
    pub async fn find_notes(&self, query: &str) -> Result<Vec<u64>, Error> {
        let params = serde_json::json! {{
            "query": query,
        }};
        self.invoke_de("findNotes", &params).await
    }

    // TODO notesInfo
    // TODO notesModTime
    // TODO deleteNotes
    // TODO removeEmptyNotes

    // Statistic actions

    /// Invokes the `getNumCardsReviewedToday` action.
    pub async fn get_num_cards_reviewed_today(&self) -> Result<u64, Error> {
        self.invoke_de_without_params("getNumCardsReviewedToday").await
    }

    // TODO getNumCardsReviewedByDay
    // TODO getCollectionStatsHTML
    // TODO cardReviews

    /// Invokes the `getReviewsOfCards` action.
    /// Returns a mapping of card ID to associated reviews.
    pub async fn get_reviews_of_cards(
        &self,
        cards: &[u64],
    ) -> Result<std::collections::HashMap<u64, Vec<ReviewOfCard>>, Error> {
        let params = serde_json::json! {{
            "cards": cards,
        }};
        self.invoke_de("getReviewsOfCards", &params).await
    }

    // TODO getLatestReviewID
    // TODO insertReviews
}
