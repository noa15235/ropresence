//! Roblox detection: running processes, local log parsing, and public API
//! lookups. Nothing here ever touches the user's Roblox cookie/token — only
//! local logs and unauthenticated public endpoints.

pub mod api;
pub mod log_parser;
pub mod process_watch;

pub use api::GameInfo;
pub use log_parser::ParsedLog;
pub use process_watch::RobloxProcesses;
