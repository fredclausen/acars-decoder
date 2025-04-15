#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Decoder {
    name: String,
    #[serde(rename = "type")]
    match_type: DecoderMatchType,
    #[serde(rename = "decodeLevel")]
    decoder_level: DecoderLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DecoderMatchType {
    PatternMatch,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DecoderLevel {
    #[default]
    None,
    Partial,
    Full,
}
