extern crate rust_line_bot_sdk;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;

use actix_web::{App, error, http, HttpMessage, HttpRequest, HttpResponse, Json, Result, server, State};
use std::env;
use std::sync::Arc;
use actix_web::FromRequest;
use std::process;
use serde_json::Number;
use rust_line_bot_sdk::event::*;
use rust_line_bot_sdk::event::message::*;

// ===== reply =====
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Reply {
    reply_token: String,
    messages: Vec<ReplyMessage>,
}

#[derive(Serialize, Debug)]
struct ReplyMessage {
    #[serde(rename = "type")]
    event_type: String,
    text: Option<String>,
}


fn _webhook(events: String, _config: State<Config>) -> Result<String> {
    println!("raw :: {}", events);
    Ok("".to_string())
}

fn webhook(events: Json<Events>, config: State<Config>) -> Result<String> {
    println!("Event :: {:#?}", events);

    let client = reqwest::Client::new();
    for event in events.events.iter() {
        let body = match event {
            Event::Message { reply_token, message, .. } => {
                let idc = String::from("i don't care");
                let s = String::from("av");
                let m = match message {
                    Message::Text { text, .. } => text,
                    Message::Image { id, .. } => id,
                    Message::Video { duration, .. } => &s,
                    Message::Audio { duration, .. } => &s,
                    Message::File { file_name, .. } => file_name,
                    Message::Location { title, .. } => title,
                    Message::Sticker { sticker_id , .. } => sticker_id,
                };
                Reply {
                    reply_token: reply_token.clone(),
                    messages: vec![
                        ReplyMessage {
                            event_type: String::from("text"),
                            text: Some(format!("hello, {}", m)),
                        },
                    ],
                }
            },
        };

        println!("Reply :: {:#?}", &body);

        let reply_url = "https://api.line.me/v2/bot/message/reply";

        let res = client
            .post(reply_url)
            .json(&body)
            .bearer_auth(config.channel_secret_token.to_string())
            .send();

        println!("Result {:#?}", res);
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
                r.method(http::Method::POST).with_config(webhook, |cfg| {
                    cfg.0.error_handler(|err, req| {
                        eprintln!("err :: {}", err);
                        error::ErrorBadRequest(err)
                    });
                }))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
