use crate::{Message, error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait GetFields
where
    Self: Serialize + Deserialize<'static>,
{
    fn get_fields(&self) -> Message;
}

impl GetFields for String {
    fn get_fields(&self) -> Message {
        let json_str = self.to_string();
        match serde_json::from_str::<Value>(&json_str) {
            Ok(message) => {
                let label = message
                    .get("label")
                    .and_then(Value::as_str)
                    .map(String::from);

                let sublabel = message
                    .get("sublabel")
                    .and_then(Value::as_str)
                    .map(String::from);

                let text = message
                    .get("text")
                    .and_then(Value::as_str)
                    .map(String::from);

                Message {
                    label,
                    sublabel,
                    text,
                }
            }
            Err(e) => {
                error!("{}", error::DecodingError::JsonParseError(e));
                Message::default()
            }
        }
    }
}
