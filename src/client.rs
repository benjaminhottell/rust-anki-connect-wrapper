use crate::endpoints::request::Request;
use crate::models::{RequestBody, ResponseBody};
use crate::error::Error;

async fn invoke<
    BodyType: serde::Serialize,
    ResultType: serde::de::DeserializeOwned,
>(
    client: &reqwest::Client,
    url: &str,
    request_body: &BodyType,
) -> Result<ResultType, Error> {
    client
        .post(url)
        .header("Content-Type", "application/json")
        .json(request_body)
        .send()
        .await
        .map_err(Error::Request)?
        .json::<ResponseBody<ResultType>>()
        .await
        .map_err(Error::DeserializeReqwest)?
        .into_result()
        .map_err(Error::Normal)
}

pub struct ClientBuilder<'a> {
    url: Option<&'a str>,
    client: Option<reqwest::Client>,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> ClientBuilder<'a> {
        ClientBuilder {
            url: None,
            client: None,
        }
    }

    pub fn with_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Client<'a> {
        Client {
            url: self.url.unwrap_or(Client::DEFAULT_URL),
            client: self.client.unwrap_or_default(),
        }
    }

}

impl<'a> Default for ClientBuilder<'a> {
    fn default() -> Self {
        ClientBuilder::new()
    }
}

/// A `Client` can invoke `Request`s.
/// `Request`s should be constructed separately.
///
/// Connecting to Anki-Connect at the default port on localhost:
///
/// ```
/// use ankiconnect::Client;
/// Client::default();
/// ```
///
/// Connecting to Anki-Connect at a custom url:
///
/// ```
/// use ankiconnect::Client;
/// let url ="http://www.example.com:8765";
/// Client::builder()
///     .with_url(url)
///     .build();
/// ```
///
/// Make sure the URL contains a scheme (e.g. `http://`)
pub struct Client<'a> {
    url: &'a str,
    client: reqwest::Client,
}

impl<'a> Client<'a> {

    pub const DEFAULT_URL: &'static str = "http://127.0.0.1:8765";

    pub fn builder() -> ClientBuilder<'a> {
        ClientBuilder::new()
    }

    pub async fn invoke_custom<
        'b,
        ParamsType: serde::Serialize,
        ResultType: serde::de::DeserializeOwned,
    >(
        &self,
        body: &RequestBody<'b, ParamsType>,
    ) -> Result<ResultType, Error> {
        invoke(&self.client, self.url, &body)
            .await
            .map(serde_json::from_value::<ResultType>)?
            .map_err(Error::DeserializeSerde)
    }

    pub async fn invoke<R: Request>(&self, request: &R) -> Result<R::Response, Error> {

        let action = request.get_action();
        let version = request.get_version();
        let params = request.get_params();

        match params {
            Some(x) => {
                let body = RequestBody::with_params(action, version, &x);
                self.invoke_custom(&body).await
            },
            _ => {
                let body = RequestBody::without_params(action, version);
                self.invoke_custom(&body).await
            },
        }
    }

    /// Invoke the API with a `Request` constructed via `default()`.
    /// Only works on `Request`s that also implement `Default`, meaning they either have no
    /// parameters or have parameters with sensible defaults.
    pub async fn invoke_default<R: Request + Default>(&self) -> Result<R::Response, Error> {
        let request = R::default();
        self.invoke(&request).await
    }

}

impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Client::builder().build()
    }
}
