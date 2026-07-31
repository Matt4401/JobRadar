use serde_json::Value;

/// Récupère un champ texte d'un objet JSON.
pub fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)?.as_str().map(|s| s.to_string())
}

/// Renvoie le premier élément si `v` est un tableau, sinon `v` lui-même.
pub fn first_item(v: &Value) -> &Value {
    match v.as_array() {
        Some(arr) => arr.first().unwrap_or(v),
        None => v,
    }
}

/// `employmentType` peut être une chaîne ou un tableau de chaînes.
pub fn employment_type(v: &Value) -> Option<String> {
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

/// Met en forme `baseSalary` (montant + devise + unité) en une chaîne lisible.
pub fn base_salary(v: &Value) -> Option<String> {
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
