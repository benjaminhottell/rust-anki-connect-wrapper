use crate::endpoints::request::Request;

// Miscellaneous Actions

// TODO requestPermission

/// Corresponds to the `version` action.
/// Returns a number indicating the version of the API server.
#[derive(Default)]
pub struct Version;

impl Request for Version {
    type Response = u64;
    type Params = ();
    fn get_action(&self) -> &'static str { "version" }
}

// TODO apiReflect

/// Corresponds to the `sync` action.
/// Prompts the running Anki instance to sync with AnkiWeb.
#[derive(Default)]
pub struct Sync;

impl Request for Sync {
    type Response = ();
    type Params = ();
    fn get_action(&self) -> &'static str { "sync" }
}

/// Corresponds to the `getProfiles` action.
/// Returns a Vec of strings indicaating the names of profiles in the client.
#[derive(Default)]
pub struct GetProfiles;

impl Request for GetProfiles {
    type Response = Vec<String>;
    type Params = ();
    fn get_action(&self) -> &'static str { "getProfiles" }
}

pub struct GetActiveProfile;

impl Request for GetActiveProfile {
    type Response = String;
    type Params = ();
    fn get_action(&self) -> &'static str { "getActiveProfile" }
}

// TODO loadProfile
// TODO multi
// TODO exportPackage
// TODO importPackage
// TODO reloadCollection
