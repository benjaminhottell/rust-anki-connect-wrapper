use strum::{EnumString, AsRefStr};

#[derive(Copy, Clone, EnumString, AsRefStr)]
#[strum(serialize_all = "camelCase")]
pub enum Order {
    Ascending,
    Descending,
}

impl serde::Serialize for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_ref())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ascending() {
        let value = Order::Ascending;
        let body = serde_json::to_value(value).unwrap();
        assert_eq!(body, serde_json::Value::String("ascending".to_string()));
    }

    #[test]
    fn serialize_descending() {
        let value = Order::Descending;
        let body = serde_json::to_value(value).unwrap();
        assert_eq!(body, serde_json::Value::String("descending".to_string()));
    }

}
