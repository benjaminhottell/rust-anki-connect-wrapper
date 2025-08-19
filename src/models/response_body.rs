use std::marker::PhantomData;

use serde::Deserialize;
use serde::de::Visitor;

/// Model to represent the response body of an API request.
/// It looks similar to a Result, but it is kept as a separate type to facilitate deserialization
pub enum ResponseBody<ResultType> {
    /// Indicates the API replied with a non-null "error" key
    Error(String),
    /// Indicates the API replied with a null "error" key, so we should inspect the "result" key
    Result(ResultType),
}

impl<ResultType> From<ResponseBody<ResultType>> for Result<ResultType, String> {
    fn from(value: ResponseBody<ResultType>) -> Self {
        match value {
            ResponseBody::Error(x) => Err(x),
            ResponseBody::Result(x) => Ok(x),
        }
    }
}

// This conversion is otherwise obnoxious to write out
impl<ResultType> ResponseBody<ResultType> {
    pub fn into_result(self) -> Result<ResultType, String> {
        self.into()
    }
}

struct ResponseBodyVisitor<ResultType> {
    marker: PhantomData<ResultType>,
}

impl<ResultType> ResponseBodyVisitor<ResultType> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'a, ResultType: Deserialize<'a>> Visitor<'a> for ResponseBodyVisitor<ResultType> {
    type Value = ResponseBody<ResultType>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map consisting of exactly two keys, error and result")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'a>, {

        // None => "error" key missing
        // Some(None) => "error" key defined with value `null`
        // Some(Some(x)) => "error" key defined with non-null value `x`
        let mut error: Option<Option<String>> = None;

        // None => "result" key missing
        // Some(x) => "result" key defined with value `x`, where `x` could be null
        let mut result: Option<ResultType> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "error" => error = Some(map.next_value()?),
                "result" => result = Some(map.next_value()?),
                unknown => return Err(serde::de::Error::unknown_field(unknown, &["error", "result"]))
            }
        }

        if error.is_none() {
            return Err(serde::de::Error::missing_field("error"));
        }

        if result.is_none() {
            return Err(serde::de::Error::missing_field("result"));
        }

        let error = error.unwrap();
        let result = result.unwrap();

        let body = match error {
            Some(x) => Self::Value::Error(x),
            _ => Self::Value::Result(result),
        };

        Ok(body)
    }
}

impl<'a, ResultType: Deserialize<'a>> Deserialize<'a> for ResponseBody<ResultType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        let visitor = ResponseBodyVisitor::<ResultType>::new();
        deserializer.deserialize_map(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn deserialize_empty_response_body() {
        let response = serde_json::json!({});
        serde_json::from_value::<ResponseBody<()>>(response)
            .unwrap()
            .into_result()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_missing_error_in_response_body() {
        let response = serde_json::json!({
            "result": 123,
        });
        serde_json::from_value::<ResponseBody<()>>(response)
            .unwrap()
            .into_result()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_missing_result_in_response_body() {
        let response = serde_json::json!({
            "error": "Made up error",
        });
        serde_json::from_value::<ResponseBody<()>>(response)
            .unwrap()
            .into_result()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_stray_key() {
        let response = serde_json::json!({
            "error": "Made up error",
            "result": null,
            "stray": "something",
        });
        serde_json::from_value::<ResponseBody<()>>(response)
            .unwrap()
            .into_result()
            .unwrap();
    }

    #[test]
    fn deserialize_null_and_null_response_body() {
        let response = serde_json::json!({
            "error": null,
            "result": null,
        });
        serde_json::from_value::<ResponseBody<()>>(response)
            .unwrap()
            .into_result()
            .unwrap();
    }

    #[test]
    fn deserialize_error() {
        let response = serde_json::json!({
            "error": "Made up error",
            "result": null,
        });
        let error = serde_json::from_value::<ResponseBody<()>>(response)
            .unwrap()
            .into_result()
            .unwrap_err();
        assert_eq!(error, "Made up error");
    }

    #[test]
    fn deserialize_result() {
        let response = serde_json::json!({
            "error": null,
            "result": "Made up result",
        });
        let body = serde_json::from_value::<ResponseBody<String>>(response)
            .unwrap()
            .into_result()
            .unwrap();
        assert_eq!(body, "Made up result");
    }

}
