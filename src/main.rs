extern crate rust_line_bot_sdk;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;

use actix_web::{App, error, http, Json, Result, server, State};
use rust_line_bot_sdk::event::message::{Message};
use rust_line_bot_sdk::event::{Events, Event};
use rust_line_bot_sdk::{LineBot};



fn _webhook(events: String, _config: State<Config>) -> Result<String> {
    println!("raw :: {}", events);
    Ok("".to_string())
}

fn webhook(events: Json<Events>, config: State<Config>) -> Result<String> {
    println!("Event :: {:#?}", events);

    let client = reqwest::Client::new();
    let line_bot = LineBot::new(config.channel_secret_token.as_str());

    for event in events.events.iter() {
        let res = match event {
            Event::Message { reply_token, message, .. } => {
                let s = String::from("av");
                let text= match message {
                    Message::Text { text, .. } => text,
                    Message::Image { id, .. } => id,
                    Message::Video { .. } => &s,
                    Message::Audio { .. } => &s,
                    Message::File { file_name, .. } => file_name,
                    Message::Location { title, .. } => title,
                    Message::Sticker { sticker_id , .. } => sticker_id,
                };

                println!("Reply :: {}", text);

                line_bot
                    .reply_text(reply_token, vec![text.as_str()])
            }
        };

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
                    cfg.0.error_handler(|err, _req| {
                        eprintln!("err :: {}", err);
                        error::ErrorBadRequest(err)
                    });
                }))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
