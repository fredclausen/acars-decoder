use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Message {
    pub(crate) label: Option<String>,
    pub(crate) sublabel: Option<String>,
    pub(crate) text: Option<String>,
}
