use crate::client::Client;
use crate::error::Error;
use crate::models::RequestBody;

impl Client {

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

}
