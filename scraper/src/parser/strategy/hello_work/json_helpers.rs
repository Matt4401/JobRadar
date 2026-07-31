use serde_json::Value;

/// Return the string value of the field `key` in the JSON object `v`, if it exists.
pub(crate) fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)?.as_str().map(|s| s.to_string())
}

/// Return the first element if `v` is an array, otherwise return `v` itself.
pub fn first_item(v: &Value) -> &Value {
    match v.as_array() {
        Some(arr) => arr.first().unwrap_or(v),
        None => v,
    }
}

/// `employmentType` can be a string or an array of strings.
pub(crate) fn employment_type(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Array(arr) => {
            let joined: Vec<String> = arr
                .iter()
                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                .collect();
            if joined.is_empty() {
                None
            } else {
                Some(joined.join(", "))
            }
        }
        _ => None,
    }
}

/// Format `baseSalary` (amount + currency + unit) into a readable string.
pub(crate) fn base_salary(v: &Value) -> Option<String> {
    let currency = v.get("currency").and_then(|c| c.as_str()).unwrap_or("");
    let amount = &v["value"];
    let value = amount
        .get("value")
        .map(|val| match val {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => String::new(),
        })
        .unwrap_or_default();
    if value.is_empty() {
        return None;
    }
    let unit = amount.get("unitText").and_then(|u| u.as_str());
    Some(match unit {
        Some(u) => format!("{value} {currency} / {u}"),
        None => format!("{value} {currency}"),
    })
}
