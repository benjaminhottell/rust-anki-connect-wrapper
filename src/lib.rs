// Relevant docs: https://git.sr.ht/~foosoft/anki-connect

pub mod models;
use models::*;

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

/// Everything that can go wrong with any request (invocation) to the Anki-Connect API
#[derive(Debug)]
pub enum Error {
    /// Occurs if there was an error making a request to the API
    Request(reqwest::Error),

    /// Occurs if the API responds to the request but does not do so in a way this API wrapper can
    /// interpret as a response model.
    DeserializeSerde(serde_json::Error),

    DeserializeReqwest(reqwest::Error),

    /// When the request is otherwise OK and everything is valid, but the API operation did not
    /// complete successfully
    Normal(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Request(e) => {
                write!(f, "error making request: {e}")
            }
            Error::DeserializeSerde(e) => {
                write!(f, "error deserializing response: {e}")
            }
            Error::DeserializeReqwest(e) => {
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

pub async fn invoke<
    BodyType: serde::Serialize,
    ResultType: serde::de::DeserializeOwned,
>(
    client: &reqwest::Client,
    url: &str,
    request_body: &BodyType,
) -> Result<ResultType, Error> {

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(request_body)
        .send()
        .await
        .map_err(Error::Request)?;

    let body = res.json::<ResponseBody<ResultType>>()
        .await
        .map_err(Error::DeserializeReqwest)?
        .into_result()
        .map_err(Error::Normal)?;

    Ok(body)
}

impl AnkiConnect {

    pub fn builder() -> AnkiConnectBuilder {
        AnkiConnectBuilder::new()
    }

    fn new(url: String, client: reqwest::Client) -> AnkiConnect {
        AnkiConnect { url, client }
    }

    pub async fn invoke<
        'a,
        ParamsType: serde::Serialize,
        ResultType: serde::de::DeserializeOwned,
    >(
        &self,
        request_body: RequestBody<'a, ParamsType>,
    ) -> Result<ResultType, Error> {
        invoke(&self.client, &self.url, &request_body)
            .await
            .map(serde_json::from_value::<ResultType>)?
            .map_err(Error::DeserializeSerde)
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
        self.invoke(RequestBody::with_optional_params("guiBrowse", options)).await
    }

    /// Invokes the `guiSelectCard` action.
    /// Expects the Card Browser dialog to already be open.
    /// Returns `true` if the Card Browser dialog is open.
    pub async fn do_gui_select_card(&self, card: &u64) -> Result<bool, Error> {
        let params = serde_json::json!({ "card": card, });
        self.invoke(RequestBody::with_params("guiDeckReview", &params)).await
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
        self.invoke(RequestBody::without_params("guiUndo")).await
    }

    /// Invokes the `guiDeckOverview` action
    /// Opens the Deck Overview dialog for the given deck.
    pub async fn do_gui_deck_overview(&self, name: &str) -> Result<bool, Error> {
        let params = serde_json::json!({ "name": name, });
        self.invoke(RequestBody::with_params("guiDeckReview", &params)).await
    }

    /// Invokes the `guiDeckBrowser` action.
    /// Opens the Deck Browser dialog.
    pub async fn do_gui_deck_browser(&self) -> Result<(), Error> {
        self.invoke(RequestBody::without_params("guiDeckBrowser")).await
    }

    /// Invokes the `guiDeckReview` action.
    /// Starts reviewing the deck. Returns `true` on success.
    pub async fn do_gui_deck_review(&self, name: &str) -> Result<bool, Error> {
        let params = serde_json::json!({ "name": name, });
        self.invoke(RequestBody::with_params("guiDeckReview", &params)).await
    }


    /// Invokes the `guiImportFile` action.
    /// Opens the Import... dialog.
    pub async fn do_gui_import_file(&self, path: Option<&str>) -> Result<(), Error> {
        match path {
            Some(x) => {
                let params = serde_json::json!({ "path": x, });
                self.invoke(RequestBody::with_params("guiImportFile", &params)).await
            },
            _ => {
                self.invoke(RequestBody::without_params("guiImportFile")).await
            },
        }
    }

    /// Invokes the `guiExitAnki` action.
    /// According to API docs, the API request will return immediately and does not wait for Anki
    // TODO guiExitAnki
    /// to actually close.
    pub async fn do_gui_exit_anki(&self) -> Result<(), Error> {
        self.invoke(RequestBody::without_params("guiExitAnki")).await
    }

    /// Invokes the `guiCheckDatabase` action.
    /// According to API docs, it should always return `true`.
    pub async fn do_gui_check_database(&self) -> Result<bool, Error> {
        self.invoke(RequestBody::without_params("guiCheckDatabase")).await
    }

    // Media actions
    // TODO

    // Miscellaneous Actions

    // TODO requestPermission

    /// Invokes the `version` action.
    /// Returns a number indicating the version of the API server.
    pub async fn get_version(&self) -> Result<u64, Error> {
        self.invoke(RequestBody::without_params("version")).await
    }

    // TODO apiReflect

    /// Invokes the `sync` action.
    /// Prompts the client to sync with AnkiWeb.
    pub async fn sync(&self) -> Result<(), Error> {
        self.invoke(RequestBody::without_params("sync")).await
    }

    /// Invokes the `getProfiles` action.
    /// Returns a Vec of strings indicating the names of profiles in the client.
    pub async fn get_profiles(&self) -> Result<Vec<String>, Error> {
        self.invoke(RequestBody::without_params("getProfiles")).await
    }

    /// Invokes the `getActiveProfile` action.
    /// Returns a string indicating the name of the active profile.
    pub async fn get_active_profile(&self) -> Result<String, Error> {
        self.invoke(RequestBody::without_params("getActiveProfile")).await
    }

    // TODO loadProfile
    // TODO multi
    // TODO exportPackage
    // TODO importPackage
    // TODO reloadCollection

    // Model actions

    /// Invokes the `modelNames` action.
    pub async fn get_model_names(&self) -> Result<String, Error> {
        self.invoke(RequestBody::without_params("modelNames")).await
    }

    /// Invokes the `modelNamesAndIds` action.
    pub async fn get_model_names_and_ids(&self) -> Result<String, Error> {
        self.invoke(RequestBody::without_params("modelNamesAndIds")).await
    }

    // TODO findModelsById
    // TODO findModelsByName
    // TODO modelFieldNames
    // TODO modelFieldDescriptions
    // TODO modelFieldFonts
    // TODO modelFieldsOnTemplates
    // TODO createModel
    // TODO modelTemplates
    // TODO modelStyling
    // TODO updateModelTemplates
    // TODO updateModelStyling
    // TODO findAndReplaceInModels
    // TODO modelTemplateRename
    // TODO modelTemplateReposition
    // TODO modelTemplateAdd
    // TODO modelTemplateRemove
    // TODO modelFieldRename
    // TODO modelFieldSetFont
    // TODO modelFieldSetFontSize
    // TODO modelFieldSetDescription
    // TODO modelFieldAdd
    // TODO modelFieldRemove
    // TODO modelFieldSetFont
    // TODO modelFieldSetFontSize
    // TODO modelFieldSetDescription

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
        self.invoke(RequestBody::with_params("addTags", &params)).await
    }

    /// Invokes the `removeTags` action.
    /// `notes` should be a Vec of note IDs.
    /// `tags` should be the tags to remove, separated by space.
    pub async fn remove_tags(&self, notes: &Vec<u64>, tags: &str) -> Result<(), Error> {
        let params = serde_json::json! {{
            "notes": notes,
            "tags": tags,
        }};
        self.invoke(RequestBody::with_params("removeTags", &params)).await
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
        self.invoke(RequestBody::with_params("findNotes", &params)).await
    }

    // TODO notesInfo
    // TODO notesModTime
    // TODO deleteNotes
    // TODO removeEmptyNotes

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
