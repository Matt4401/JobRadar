use serde::Serialize;

/// Ties a Rust model to its SQL table.
pub trait Table: Serialize + Default {
    const TABLE: &'static str;

    fn columns() -> Vec<String> {
        match serde_json::to_value(Self::default()) {
            Ok(serde_json::Value::Object(map)) => map.into_iter().map(|(key, _)| key).collect(),
            _ => Vec::new(),
        }
    }
}
