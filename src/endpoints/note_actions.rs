use crate::endpoints::request::Request;

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

/// Corresponds to the `addTags` action.
/// `notes` should be a Vec of note IDs.
/// `tags` should be the tags to add, separated by space.
#[derive(serde::Serialize)]
pub struct AddTags<'a> {
    notes: &'a [u64],
    tags: &'a str,
}

impl<'a> AddTags<'a> {
    pub fn new(notes: &'a [u64], tags: &'a str) -> Self {
        Self {
            notes,
            tags,
        }
    }
}

impl<'a> Request for AddTags<'a> {
    type Response = ();
    type Params = Self;
    fn get_action(&self) -> &'static str { "addTags" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `removeTags` action.
/// `notes` should be a Vec of note IDs.
/// `tags` should be the tags to add, separated by space.
#[derive(serde::Serialize)]
pub struct RemoveTags<'a> {
    notes: &'a [u64],
    tags: &'a str,
}

impl<'a> Request for RemoveTags<'a> {
    type Response = ();
    type Params = Self;
    fn get_action(&self) -> &'static str { "removeTags" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO getTags
// TODO clearUnusedTags
// TODO replaceTags
// TODO replaceTagsInAllNotes

/// Corresponds to the `findNotes` action.
/// Returns note IDs for notes that match the given query.
/// See: <https://docs.ankiweb.net/searching.html>
#[derive(serde::Serialize)]
pub struct FindNotes<'a> {
    query: &'a str,
}

impl<'a> Request for FindNotes<'a> {
    type Response = ();
    type Params = Self;
    fn get_action(&self) -> &'static str { "findNotes" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO notesInfo
// TODO notesModTime
// TODO deleteNotes
// TODO removeEmptyNotes
