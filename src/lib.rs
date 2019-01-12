#[macro_use]
extern crate serde_derive;
extern crate serde_json;


pub mod event;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Reply {
    reply_token: String,
    messages: Vec<ReplyMessage>,
}

#[derive(Serialize, Debug)]
#[serde(rename = "camelCase")]
struct ReplyMessage {
    #[serde(rename = "type")]
    reply_type: String,
    text: String,
}

pub struct LineBot<'a> {
    channel_secret_token: &'a str,
    http_client: reqwest::Client,
}

static REPLY_API_ENDPOINT: &str = "https://api.line.me/v2/bot/message/reply";

impl<'a> LineBot<'a> {
    pub fn new(channel_secret_token: &str) -> LineBot {
        LineBot {
            channel_secret_token,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn reply_text(&self, reply_token: &str, messages: Vec<&str>) -> Result<reqwest::Response, reqwest::Error> {
        let messages = messages
            .iter()
            .map(|text| ReplyMessage {
                reply_type: "text".to_string(),
                text: text.to_string(),
            })
            .collect();

        let reply_token = reply_token.to_string();

        let body = Reply { reply_token, messages };

        self.http_client
            .post(REPLY_API_ENDPOINT)
            .json(&body)
            .bearer_auth(self.channel_secret_token.to_string())
            .send()
    }
}
