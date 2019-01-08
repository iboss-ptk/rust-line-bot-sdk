use serde_json::Number;
use self::message::{Message};

pub mod message;

#[derive(Deserialize, Debug)]
pub struct Events {
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "message", rename_all = "camelCase")]
    Message {
        reply_token: String,
        timestamp: Number,
        message: Message,
    }
}
