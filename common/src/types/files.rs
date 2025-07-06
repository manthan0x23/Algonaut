use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "text")]
    Text,

    #[serde(rename = "image")]
    Image,

    #[serde(rename = "video")]
    Video,

    #[serde(rename = "file")]
    File,
}
