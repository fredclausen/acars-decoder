use crate::common::{decoded_message::DecodedMessage, internal_message::Message};
use crate::plugins::DecoderPluginInterface;

#[derive(Debug, Clone)]
pub struct Plugin1J2JFTX {
    pub name: String,
    pub qualifiers: Vec<String>,
}

impl Plugin1J2JFTX {
    pub fn new() -> Self {
        Plugin1J2JFTX {
            name: "Plugin 1J/2J FTX".to_string(),
            qualifiers: vec!["1J".to_string(), "2J".to_string()],
        }
    }
}

impl DecoderPluginInterface for Plugin1J2JFTX {
    fn decode(&self, message: &Message) -> DecodedMessage {
        // Decode the message here
        let decoded_message = DecodedMessage::default();
        // Add decoding logic here
        decoded_message
    }

    fn qualifiers(&self) -> Vec<String> {
        self.qualifiers.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn box_clone(&self) -> Box<dyn DecoderPluginInterface> {
        Box::new(self.clone())
    }
}
