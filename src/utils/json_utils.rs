use serde_json::Value;

pub fn convert_to_json<T>(data: Vec<T>) -> serde_json::Value
    where
        T: serde::Serialize,
{
    data.into_iter().map(|item| serde_json::json!(item)).collect()
}
