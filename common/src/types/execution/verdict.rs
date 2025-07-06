use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// Pending (received, not yet queued)
    #[serde(rename = "PD")]
    PD,

    /// In Queue (waiting to be judged)
    #[serde(rename = "QU")]
    QU,

    /// Running (currently being judged)
    #[serde(rename = "RN")]
    RN,

    /// Accepted
    #[serde(rename = "AC")]
    AC,

    /// Wrong Answer
    #[serde(rename = "WA")]
    WA,

    /// Time Limit Exceeded
    #[serde(rename = "TLE")]
    TLE,

    /// Memory Limit Exceeded
    #[serde(rename = "MLE")]
    MLE,

    /// Runtime Error
    #[serde(rename = "RE")]
    RE,

    /// Compilation Error
    #[serde(rename = "CE")]
    CE,

    /// Output Limit Exceeded
    #[serde(rename = "OLE")]
    OLE,

    /// Presentation Error
    #[serde(rename = "PE")]
    PE,

    /// Internal Error (judge system error)
    #[serde(rename = "IE")]
    IE,

    /// System Error (system/server failure)
    #[serde(rename = "SE")]
    SE,

    /// Rejected (manual rejection/policy violation)
    #[serde(rename = "RJ")]
    RJ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    #[serde(rename = "Accepted")]
    Accepted,

    #[serde(rename = "Wrong Answer")]
    WrongAnswer,

    #[serde(rename = "Time Limit Exceeded")]
    TimeLimitExceeded,

    #[serde(rename = "Memory Limit Exceeded")]
    MemoryLimitExceeded,

    #[serde(rename = "Runtime Error")]
    RuntimeError,

    #[serde(rename = "Compilation Error")]
    CompilationError,

    #[serde(rename = "Output Limit Exceeded")]
    OutputLimitExceeded,

    #[serde(rename = "Presentation Error")]
    PresentationError,

    #[serde(rename = "Internal Error")]
    InternalError,

    #[serde(rename = "System Error")]
    SystemError,

    #[serde(rename = "Rejected")]
    Rejected,

    #[serde(rename = "Pending")]
    Pending,

    #[serde(rename = "In Queue")]
    InQueue,

    #[serde(rename = "Running")]
    Running,

    #[serde(rename = "Plagiarized")]
    Plagiarized,

    #[serde(rename = "Skipped")]
    Skipped,
}
