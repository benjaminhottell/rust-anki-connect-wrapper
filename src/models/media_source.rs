use std::borrow::Cow;

/// Represents the source of a media file, which could be an absolute path to a local file
/// (relative to the Anki-Connect server), a URL to another server which is hosting the file, or a
/// base64 encoded string.
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum MediaSource<'a> {

    /// Represents the case where the media source is a base64-encoded string, inline with the
    /// request.
    #[serde(rename = "data")]
    Data(Cow<'a, String>),

    /// Represents the case where the media source is a file on the same filesystem accessible to
    /// Anki-Connect. This should be an absolute path to that file.
    #[serde(rename = "path")]
    Path(Cow<'a, String>),

    /// Represents the case where the media source is hosted on another server, and that
    /// Anki-Connect should connect to this URL to download it.
    #[serde(rename = "url")]
    Url(Cow<'a, String>),

}
