use strum::{EnumString, AsRefStr};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_note_creation() {
        let value = BrowserColumn::NoteCreation;
        let body = serde_json::to_value(&value).unwrap();
        assert_eq!(body, serde_json::Value::String("noteCrt".to_string()));
    }

}
