use std::collections::HashMap;

use crate::common::{decoded_message::DecodedMessage, internal_message::Message};

trait DecoderPluginInterface
where
    Self: Clone,
{
    /// Decode a message using the plugin.
    fn decode(&self, message: &Message) -> DecodedMessage;
    fn qualifiers(&self) -> Vec<String>;
}

struct PluginManager<T> {
    // FIXME: can we do this without cloning?
    plugins: HashMap<String, T>,
}

impl<T: DecoderPluginInterface> PluginManager<T> {
    fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    fn add_plugin(&mut self, plugin: T) {
        let qualifiers = plugin.qualifiers();
        for qualifier in qualifiers {
            self.plugins.insert(qualifier, plugin.clone());
        }
    }

    fn decode(&self, message: &Message) -> DecodedMessage {
        if let Some(label) = &message.label {
            if let Some(plugin) = self.plugins.get(label.as_str()) {
                return plugin.decode(message);
            }
        }
        DecodedMessage::default()
    }
}
