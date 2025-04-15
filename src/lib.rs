#![warn(clippy::pedantic)]

#[macro_use]
extern crate log;

pub mod common;
pub mod error;

#[cfg(feature = "acars_parser")]
use acars_vdlm2_parser::AcarsVdlm2Message;
use anyhow::Result;
use common::internal_message::Message;
use error::DecodingError;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Default, Eq, PartialEq)]
enum ACARSMessageType {
    String(String),
    StringParts(Option<String>, Option<String>, Option<String>),
    #[cfg(feature = "acars_parser")]
    AcarsVdlm2Message(AcarsVdlm2Message),
    #[default]
    Unknown,
}

impl ACARSMessageType {
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, ACARSMessageType::String(_))
    }

    #[cfg(feature = "acars_parser")]
    #[must_use]
    pub fn is_acars_vdlm2_message(&self) -> bool {
        matches!(self, ACARSMessageType::AcarsVdlm2Message(_))
    }

    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, ACARSMessageType::Unknown)
    }

    #[must_use]
    pub fn get_string(&self) -> Option<&String> {
        if let ACARSMessageType::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct DecodedACARSMessage {
    message: Message,
    decoded: Option<String>,
}

impl DecodedACARSMessage {
    #[must_use]
    pub fn new(message: &ACARSMessageType) -> Result<Self> {
        match message {
            ACARSMessageType::String(s) => match serde_json::from_str::<Value>(s) {
                Ok(value) => Ok(DecodedACARSMessage {
                    message: Message {
                        label: value
                            .get("label")
                            .and_then(|v| v.as_str())
                            .map(std::string::ToString::to_string),
                        sublabel: value
                            .get("sublabel")
                            .and_then(|v| v.as_str())
                            .map(std::string::ToString::to_string),
                        text: value
                            .get("text")
                            .and_then(|v| v.as_str())
                            .map(std::string::ToString::to_string),
                    },
                    decoded: Some(s.clone()),
                }),
                Err(e) => Err(DecodingError::JsonParseError(e).into()),
            },
            ACARSMessageType::StringParts(label, sublabel, text) => Ok(DecodedACARSMessage {
                message: Message {
                    label: label.clone(),
                    sublabel: sublabel.clone(),
                    text: text.clone(),
                },
                decoded: text.clone(),
            }),
            #[cfg(feature = "acars_parser")]
            ACARSMessageType::AcarsVdlm2Message(_) => todo!(),
            ACARSMessageType::Unknown => Err(anyhow::anyhow!("Unknown ACARS message type")),
        }
    }

    /// Decodes the ACARS message.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `acars_parser` feature is enabled
    /// and the message cannot be parsed into an `AcarsVdlm2Message`.
    pub fn decode(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acars_vdlm2_parser::AcarsVdlm2Message;

    #[derive(Deserialize, Debug)]
    struct TestMessage {
        label: Option<String>,
        sublabel: Option<String>,
        text: Option<String>,
    }

    #[test]
    fn test_acars_message_type_from_string_ok() {
        let json_str =
            r#"{"label": "test_label", "sublabel": "test_sublabel", "text": "test_text"}"#;
        let message: ACARSMessageType = ACARSMessageType::String(json_str.to_string());
        let decoder = DecodedACARSMessage::new(&message);
        assert!(decoder.is_ok());
    }

    #[test]
    fn test_acars_message_type_from_string_not_okay() {
        let json_str =
            r#"{"label": "test_label", "sublabel": "test_sublabel", "text": "test_text""#;
        let message: ACARSMessageType = ACARSMessageType::String(json_str.to_string());
        let decoder = DecodedACARSMessage::new(&message);
        assert!(decoder.is_err());
        if let Err(e) = decoder {
            assert_eq!(
                e.to_string(),
                "Failed to parse string in to JSON. Serde says: EOF while parsing an object at line 1 column 72"
            );
        }
    }
}
