use std::borrow::Cow;
use crate::{
    endpoints::request::Request,
    models::MediaSource,
};

// Media actions

/// Corresponds to the `storeMediaFile` action.
#[derive(serde::Serialize)]
pub struct StoreMediaFile<'a> {
    filename: Cow<'a, str>,
    source: MediaSource<'a>,

    #[serde(
        rename = "deleteExisting",
        skip_serializing_if = "Option::is_none",
    )]
    delete_existing: Option<bool>,

    #[serde(
        rename = "skipHash",
        skip_serializing_if = "Option::is_none",
    )]
    skip_hash: Option<bool>,

}

impl<'a> Request for StoreMediaFile<'a> {
    type Response = String;
    type Params = Self;
    fn get_action(&self) -> &'static str { "storeMediaFile" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `retrieveMediaFile` action.
#[derive(serde::Serialize)]
pub struct RetrieveMediaFile<'a> {
    filename: Cow<'a, str>,
}

impl<'a> Request for RetrieveMediaFile<'a> {
    type Response = String;
    type Params = Self;
    fn get_action(&self) -> &'static str { "retrieveMediaFile" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `getMediaFilesNames` action.
#[derive(serde::Serialize)]
pub struct GetMediaFilesNames<'a> {
    // The docs suggest there is a default behavior of fetching all items, but in practice passing
    // None causes an error on the Anki-Connect server.
    pattern: Cow<'a, str>,
}

impl<'a> GetMediaFilesNames<'a> {
    pub fn new(pattern: impl Into<Cow<'a, str>>) -> Self {
        Self {
            pattern: pattern.into(),
        }
    }
    pub fn all() -> Self {
        Self::new("*")
    }
}

impl<'a> Default for GetMediaFilesNames<'a> {
    fn default() -> Self {
        Self::all()
    }
}

impl<'a> Request for GetMediaFilesNames<'a> {
    type Params = Self;
    type Response = Vec<String>;
    fn get_action(&self) -> &'static str { "getMediaFilesNames" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

/// Corresponds to the `getMediaDirPath` action.
#[derive(Default)]
pub struct GetMediaDirPath;

impl Request for GetMediaDirPath {
    type Params = ();
    type Response = String;
    fn get_action(&self) -> &'static str { "getMediaDirPath" }
}

/// Corresponds to the `deleteMediaFile` action.
#[derive(serde::Serialize)]
pub struct DeleteMediaFile<'a> {
    filename: Cow<'a, str>,
}

impl<'a> DeleteMediaFile<'a> {
    pub fn new(filename: impl Into<Cow<'a, str>>) -> Self {
        Self {
            filename: filename.into(),
        }
    }
}

impl<'a> Request for DeleteMediaFile<'a> {
    type Params = Self;
    type Response = ();
    fn get_action(&self) -> &'static str { "deleteMediaFile" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}
