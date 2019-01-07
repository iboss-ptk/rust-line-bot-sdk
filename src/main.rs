extern crate actix_web;
#[macro_use]
extern crate serde_derive;

use actix_web::{App, error, http, HttpMessage, HttpRequest, HttpResponse, Json, Result, State, server};
use std::env;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct Message {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct MessageEvent {
    #[serde(rename = "replyToken")]
    reply_token: String,
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Events {
    events: Vec<MessageEvent>,
}

#[derive(Serialize, Debug)]
struct ReplyMessage {
    #[serde(rename = "type")]
    event_type: String,
    text: Option<String>,
}

#[derive(Serialize, Debug)]
struct Reply {
    #[serde(rename = "replyToken")]
    reply_token: String,
    messages: Vec<ReplyMessage>,
}

fn webhook(events: Json<Events>, config: State<Config>) -> Result<String> {
    println!("{:?}", events);

    let client = reqwest::Client::new();
    for event in events.events.iter() {
        let body = Reply {
            reply_token: event.reply_token.clone(),
            messages: vec![
                ReplyMessage {
                    event_type: String::from("text"),
                    text: Some(String::from("hello")),
                },
            ],
        };

        println!("body: {}", serde_json::to_string(&body).unwrap());

        let reply_url = "https://api.line.me/v2/bot/message/reply";

        let res = client
            .post(reply_url)
            .json(&body)
            .bearer_auth(config.channel_secret_token.to_string())
            .send();

        println!("result {:?}", res);
    }

    Ok(String::from("ok"))
}

fn get_secret() -> String {
    std::env::var("CHANNEL_SECRET_TOKEN")
        .expect("CHANNEL_SECRET_TOKEN environment variable not found")
}

#[derive(Clone)]
struct Config {
    channel_secret_token: String,
}

fn main() {
    let secret = get_secret();

    server::new(move || {
        App::with_state(Config { channel_secret_token: secret.clone() })
            .resource("/webhook", |r|
                r.method(http::Method::POST).with(webhook))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
