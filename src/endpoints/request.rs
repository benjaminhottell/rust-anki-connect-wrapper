const DEFAULT_API_VERSION: u32 = 6;

pub trait Request {

    /// The response object that the API will return in the `result` field.
    /// Use unit type () if it always returns `null`.
    type Response: serde::de::DeserializeOwned;

    type Params: serde::Serialize;

    /// The action that will be invoked as defined in the API specification
    fn get_action(&self) -> &'static str;

    /// The version to specify in the request.
    fn get_version(&self) -> u32 { DEFAULT_API_VERSION }

    /// The parameters that should appear in the `params` field of the request.
    /// If `None`, then the `params` field will be omitted.
    /// The default implementation returns None.
    fn get_params(&self) -> Option<&Self::Params> { None }

}
