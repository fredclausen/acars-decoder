#![warn(clippy::pedantic)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

pub mod common;
pub mod error;

#[cfg(feature = "acars_parser")]
use acars_vdlm2_parser::AcarsVdlm2Message;

use common::{decoded_message::DecodedMessage, get_fields::GetFields, internal_message::Message};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MessageDecoder {}

impl MessageDecoder {
    #[must_use]
    pub fn new() -> Self {
        MessageDecoder {}
    }

    #[must_use]
    pub fn decode<T>(&self, message: &T) -> DecodedMessage
    where
        T: GetFields,
    {
        DecodedMessage::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::decoded_message;

    use super::*;
    use acars_vdlm2_parser::AcarsVdlm2Message;

    #[test]
    fn get_fields_from_string_ok() {
        let json_str =
            r#"{"label": "test_label", "sublabel": "test_sublabel", "text": "test_text"}"#;

        let message: Message = json_str.to_string().get_fields();
        assert_eq!(message.label, Some("test_label".to_string()));
        assert_eq!(message.sublabel, Some("test_sublabel".to_string()));
        assert_eq!(message.text, Some("test_text".to_string()));
    }

    #[test]
    fn get_fields_from_string_error() {
        let json_str =
            r#"{"label": "test_label", "sublabel": "test_sublabel", "text": "test_text""#;

        let message: Message = json_str.to_string().get_fields();
        assert_eq!(message.label, None);
        assert_eq!(message.sublabel, None);
        assert_eq!(message.text, None);
    }
}
