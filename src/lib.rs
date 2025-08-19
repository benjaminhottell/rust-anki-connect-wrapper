// Relevant docs: https://git.sr.ht/~foosoft/anki-connect

use strum::{EnumString, AsRefStr};

/// The full URL that Anki-Connect runs on by default
pub const DEFAULT_URL: &str = "http://127.0.0.1:8765";

const API_VERSION: u32 = 6;

#[derive(serde::Serialize)]
pub struct RequestBody<'a, ParamsType: serde::Serialize> {
    action: &'a str,
    version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<&'a ParamsType>,
}

impl<'a, ParamsType: serde::Serialize> RequestBody<'a, ParamsType> {

    pub fn with_params(action: &'a str, params: &'a ParamsType) -> Self {
        Self {
            action,
            params: Some(params),
            version: API_VERSION,
        }
    }

    pub fn with_optional_params(action: &'a str, params: Option<&'a ParamsType>) -> Self {
        Self {
            action,
            params,
            version: API_VERSION,
        }
    }

}

impl<'a> RequestBody<'a, ()> {
    pub fn without_params(action: &'a str) -> RequestBody<'a, ()> {
        RequestBody::<'a, ()> {
            action,
            params: None,
            version: API_VERSION,
        }
    }
}

#[derive(EnumString, AsRefStr)]
#[strum(serialize_all = "camelCase")]
pub enum Order {
    Ascending,
    Descending,
}

impl serde::Serialize for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_ref())
    }
}

/// I mimic Anki's original enum members and serialization rules as closely as possible.
/// See: https://github.com/ankitects/anki/blob/main/rslib/src/browser_table.rs
#[derive(EnumString, AsRefStr)]
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

impl serde::Serialize for BrowserColumn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_ref())
    }
}

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

/// Represents data returned by the `getReviewsOfCards` action.
/// A few fields are renamed to better adhere to Rust conventions and avoid keywords.
#[derive(serde::Deserialize)]
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

    pub async fn invoke<'a, ParamsType: serde::Serialize>(
        &self,
        request_body: RequestBody<'a, ParamsType>,
    ) -> Result<serde_json::Value, Error> {

        let res = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
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

    pub async fn invoke_de<
        'a,
        ParamsType: serde::Serialize,
        ResultType: serde::de::DeserializeOwned,
    >(
        &self,
        request_body: RequestBody<'a, ParamsType>,
    ) -> Result<ResultType, Error> {
        self.invoke(request_body)
            .await
            .map(serde_json::from_value::<ResultType>)?
            .map_err(DeserializeError::Serde)
            .map_err(Error::Deserialize)
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
        self.invoke_de(RequestBody::with_params("findCards", &params)).await
    }

    /// Invokes the `cardsToNotes` action.
    /// Returns all note IDs corresponding to the given card IDs.
    /// The order of elements in the output array is unspecified.
    pub async fn get_notes_from_cards(&self, cards: &Vec<u64>) -> Result<Vec<u64>, Error> {
        let params = serde_json::json! {{
            "cards": cards,
        }};
        self.invoke_de(RequestBody::with_params("cardsToNotes", &params)).await
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
        options: Option<&GuiBrowseOptions>,
    ) -> Result<Vec<u64>, Error> {
        self.invoke_de(RequestBody::with_optional_params("guiBrowse", options)).await
    }

    /// Invokes the `guiSelectCard` action.
    /// Expects the Card Browser dialog to already be open.
    /// Returns `true` if the Card Browser dialog is open.
    pub async fn do_gui_select_card(&self, card: &u64) -> Result<bool, Error> {
        let params = serde_json::json!({ "card": card, });
        self.invoke_de(RequestBody::with_params("guiDeckReview", &params)).await
    }

    // TODO guiSelectedNotes
    // TODO guiAddCards
    // TODO guiEditNote
    // TODO guiCurrentCard
    // TODO guiStartCardTimer
    // TODO guiShowQuestion
    // TODO guiShowAnswer
    // TODO guiAnswerCard

    /// Invokes the `guiUndo` action.
    /// Returns `true` on success.
    pub async fn do_gui_undo(&self) -> Result<bool, Error> {
        self.invoke_de(RequestBody::without_params("guiUndo")).await
    }

    /// Invokes the `guiDeckOverview` action
    /// Opens the Deck Overview dialog for the given deck.
    pub async fn do_gui_deck_overview(&self, name: &str) -> Result<bool, Error> {
        let params = serde_json::json!({ "name": name, });
        self.invoke_de(RequestBody::with_params("guiDeckReview", &params)).await
    }

    /// Invokes the `guiDeckBrowser` action.
    /// Opens the Deck Browser dialog.
    pub async fn do_gui_deck_browser(&self) -> Result<(), Error> {
        self.invoke_de(RequestBody::without_params("guiDeckBrowser")).await
    }

    /// Invokes the `guiDeckReview` action.
    /// Starts reviewing the deck. Returns `true` on success.
    pub async fn do_gui_deck_review(&self, name: &str) -> Result<bool, Error> {
        let params = serde_json::json!({ "name": name, });
        self.invoke_de(RequestBody::with_params("guiDeckReview", &params)).await
    }


    /// Invokes the `guiImportFile` action.
    /// Opens the Import... dialog.
    pub async fn do_gui_import_file(&self, path: Option<&str>) -> Result<(), Error> {
        match path {
            Some(x) => {
                let params = serde_json::json!({ "path": x, });
                self.invoke_de(RequestBody::with_params("guiImportFile", &params)).await
            },
            _ => {
                self.invoke_de(RequestBody::without_params("guiImportFile")).await
            },
        }
    }

    /// Invokes the `guiExitAnki` action.
    /// According to API docs, the API request will return immediately and does not wait for Anki
    // TODO guiExitAnki
    /// to actually close.
    pub async fn do_gui_exit_anki(&self) -> Result<(), Error> {
        self.invoke_de(RequestBody::without_params("guiExitAnki")).await
    }

    /// Invokes the `guiCheckDatabase` action.
    /// According to API docs, it should always return `true`.
    pub async fn do_gui_check_database(&self) -> Result<bool, Error> {
        self.invoke_de(RequestBody::without_params("guiCheckDatabase")).await
    }

    // Media actions
    // TODO

    // Miscellaneous Actions

    // TODO requestPermission

    /// Invokes the `version` action.
    /// Returns a number indicating the version of the API server.
    pub async fn get_version(&self) -> Result<u64, Error> {
        self.invoke_de(RequestBody::without_params("version")).await
    }

    // TODO apiReflect

    /// Invokes the `sync` action.
    /// Prompts the client to sync with AnkiWeb.
    pub async fn sync(&self) -> Result<(), Error> {
        self.invoke_de(RequestBody::without_params("sync")).await
    }

    /// Invokes the `getProfiles` action.
    /// Returns a Vec of strings indicating the names of profiles in the client.
    pub async fn get_profiles(&self) -> Result<Vec<String>, Error> {
        self.invoke_de(RequestBody::without_params("getProfiles")).await
    }

    /// Invokes the `getActiveProfile` action.
    /// Returns a string indicating the name of the active profile.
    pub async fn get_active_profile(&self) -> Result<String, Error> {
        self.invoke_de(RequestBody::without_params("getActiveProfile")).await
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

    /// Invokes the `addTags` action.
    /// `notes` should be a Vec of note IDs.
    /// `tags` should be the tags to add, separated by space.
    pub async fn add_tags(&self, notes: &Vec<u64>, tags: &str) -> Result<(), Error> {
        let params = serde_json::json! {{
            "notes": notes,
            "tags": tags,
        }};
        self.invoke_de(RequestBody::with_params("addTags", &params)).await
    }

    /// Invokes the `removeTags` action.
    /// `notes` should be a Vec of note IDs.
    /// `tags` should be the tags to remove, separated by space.
    pub async fn remove_tags(&self, notes: &Vec<u64>, tags: &str) -> Result<(), Error> {
        let params = serde_json::json! {{
            "notes": notes,
            "tags": tags,
        }};
        self.invoke_de(RequestBody::with_params("removeTags", &params)).await
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
        self.invoke_de(RequestBody::with_params("findNotes", &params)).await
    }

    // TODO notesInfo
    // TODO notesModTime
    // TODO deleteNotes
    // TODO removeEmptyNotes

    // Statistic actions

    /// Invokes the `getNumCardsReviewedToday` action.
    pub async fn get_num_cards_reviewed_today(&self) -> Result<u64, Error> {
        self.invoke_de(RequestBody::without_params("getNumCardsReviewedToday")).await
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
        self.invoke_de(RequestBody::with_params("getReviewsOfCards", &params)).await
    }

    // TODO getLatestReviewID
    // TODO insertReviews
}
