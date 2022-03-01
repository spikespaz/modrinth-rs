use form_urlencoded::Serializer as QuerySerializer;
use serde::Serialize;
use serde_json::Value as JsonValue;

pub trait JsonQueryParams<'a>
where
    Self: 'a + Serialize,
{
    fn to_query_string(&'a self) -> String {
        let mut encoder = QuerySerializer::new(String::new());

        let object = match serde_json::to_value(self).unwrap() {
            JsonValue::Object(object) => object,
            _ => panic!("expected a JSON object"),
        };

        for (key, value) in object {
            if value == JsonValue::Null {
                continue;
            }

            encoder.append_pair(&key, &value.to_string());
        }

        encoder.finish()
    }
}

pub type SearchFilters<T> = Vec<Vec<T>>;
