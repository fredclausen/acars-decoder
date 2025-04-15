use std::collections::HashMap;

use crate::common::{decoded_message::DecodedMessage, internal_message::Message};

pub trait DecoderPluginInterface {
    /// Decode a message using the plugin.
    fn decode(&self, message: &Message) -> DecodedMessage;
    fn qualifiers(&self) -> Vec<String>;
    fn name(&self) -> String;
    fn box_clone(&self) -> Box<dyn DecoderPluginInterface>;
}

impl std::fmt::Debug for dyn DecoderPluginInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Removed the Debug implementation for PluginManagerType as it is unnecessary.
        write!(f, "Plugin {}", self.name())
    }
}

impl Clone for Box<dyn DecoderPluginInterface> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub type PluginManagerType = Box<dyn DecoderPluginInterface>;

#[derive(Debug, Default)]
pub struct PluginManager {
    // FIXME: can we do this without cloning?
    plugins: HashMap<String, PluginManagerType>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    fn add_plugin(&mut self, plugin: PluginManagerType) {
        let qualifiers = plugin.qualifiers();
        for qualifier in qualifiers {
            self.plugins.insert(qualifier, plugin.clone());
        }
    }

    pub fn decode(&self, message: &Message) -> DecodedMessage {
        if let Some(label) = &message.label {
            if let Some(plugin) = self.plugins.get(label.as_str()) {
                return plugin.decode(message);
            }
        }
        DecodedMessage::default()
    }
}
