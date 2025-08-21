use crate::client::Client;
use crate::error::Error;
use crate::models::{RequestBody, GuiBrowseOptions};

impl Client {

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

}
