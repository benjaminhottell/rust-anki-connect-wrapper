use serde;

#[derive(serde::Serialize)]
pub struct RequestBody<'a, ParamsType: serde::Serialize> {
    action: &'a str,
    version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<&'a ParamsType>,
}

impl<'a> RequestBody<'a, ()> {
    pub fn without_params(action: &'a str, version: u32) -> RequestBody<'a, ()> {
        RequestBody::<'a, ()> {
            action,
            version,
            params: None,
        }
    }
}

impl<'a, ParamsType: serde::Serialize> RequestBody<'a, ParamsType> {
    pub fn with_params(
        action: &'a str,
        version: u32,
        params: &'a ParamsType,
    ) -> Self {
        Self {
            action,
            version,
            params: Some(params),
        }
    }
}

