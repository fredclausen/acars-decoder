use super::decoder::Decoder;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DecodedMessage {
    pub decoded: bool,
    pub decoder: Decoder,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Formatted {
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Items {
    items: Vec<EachItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EachItem {
    #[serde(rename = "type")]
    item_type: String,
    code: String,
    label: String,
    value: String,
}
