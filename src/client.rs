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

pub struct ClientBuilder {
    url: Option<String>,
    client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder{
        ClientBuilder {
            url: None,
            client: None,
        }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Client {
        Client {
            url: self.url.unwrap_or_else(|| Client::DEFAULT_URL.to_string()),
            client: self.client.unwrap_or_default(),
        }
    }

}

impl Default for ClientBuilder {
    fn default() -> Self {
        ClientBuilder::new()
    }
}

pub struct Client {
    url: String,
    client: reqwest::Client,
}

impl Client {

    pub const DEFAULT_URL: &'static str = "http://127.0.0.1:8765";

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn invoke_custom<
        'a,
        ParamsType: serde::Serialize,
        ResultType: serde::de::DeserializeOwned,
    >(
        &self,
        body: &RequestBody<'a, ParamsType>,
    ) -> Result<ResultType, Error> {
        invoke(&self.client, &self.url, &body)
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

}
