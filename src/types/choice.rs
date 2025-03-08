use super::Message;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}
