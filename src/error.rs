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
