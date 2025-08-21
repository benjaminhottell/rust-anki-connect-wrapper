use serde;

const API_VERSION: u32 = 6;

#[derive(serde::Serialize)]
pub struct RequestBody<'a, ParamsType: serde::Serialize> {
    action: &'a str,
    version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<&'a ParamsType>,
}

impl<'a, ParamsType: serde::Serialize> RequestBody<'a, ParamsType> {

    pub fn with_params(action: &'a str, params: &'a ParamsType) -> Self {
        Self {
            action,
            params: Some(params),
            version: API_VERSION,
        }
    }

    pub fn with_optional_params(action: &'a str, params: Option<&'a ParamsType>) -> Self {
        Self {
            action,
            params,
            version: API_VERSION,
        }
    }

}

impl<'a> RequestBody<'a, ()> {
    pub fn without_params(action: &'a str) -> RequestBody<'a, ()> {
        RequestBody::<'a, ()> {
            action,
            params: None,
            version: API_VERSION,
        }
    }
}
