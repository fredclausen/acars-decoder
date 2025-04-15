use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Message {
    pub(crate) label: Option<String>,
    pub(crate) sublabel: Option<String>,
    pub(crate) text: Option<String>,
}

impl Message {
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.label.is_none() && self.sublabel.is_none() && self.text.is_none()
    }
}
