use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "cpp20")]
    Cpp20,

    #[serde(rename = "python3")]
    Python3,

    #[serde(rename = "java")]
    Java,

    #[serde(rename = "javascript")]
    Javascript,
}
