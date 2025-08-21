use crate::client::Client;
use crate::error::Error;
use crate::models::RequestBody;

impl Client {

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

}
