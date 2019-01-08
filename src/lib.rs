#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text {
        id: String,
        text: String,
    },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        id: String,
        content_provider: ContentProvider,
    },
    #[serde(rename = "video", rename_all = "camelCase")]
    Video {
        id: String,
        duration: u64,
        content_provider: ContentProvider,
    },
    #[serde(rename = "audio", rename_all = "camelCase")]
    Audio {
        id: String,
        duration: u64,
        content_provider: ContentProvider,
    },
    #[serde(rename = "file", rename_all = "camelCase")]
    File {
        id: String,
        file_name: String,
        file_size: u64,
    },
    #[serde(rename = "sticker", rename_all = "camelCase")]
    Sticker {
        id: String,
        package_id: String,
        sticker_id: String,
    },
}


#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum ContentProvider {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "external", rename_all = "camelCase")]
    External {
        original_content_url: String,
        preview_image_url: String,
    },
}

#[cfg(test)]
mod test {
    use super::*;

    mod message {
        use super::*;

        #[test]
        fn test_deserialize_text_message() {
            let json = r#"
            {
                "id": "325708",
                "type": "text",
                "text": "Hello, world!"
            }
            "#;
            let res: Message = serde_json::from_str(json).expect("not formatted properly");

            assert_eq!(res, Message::Text {
                id: String::from("325708"),
                text: String::from("Hello, world!"),
            });
        }

        #[test]
        fn test_deserialize_image_message_from_line() {
            let json = r#"
            {
                "id": "325708",
                "type": "image",
                "contentProvider": {
                    "type": "line"
                }
            }
            "#;
            let res: Message = serde_json::from_str(json).expect("not formatted properly");

            assert_eq!(res, Message::Image {
                id: String::from("325708"),
                content_provider: ContentProvider::Line,
            });
        }

        #[test]
        fn test_deserialize_image_message_from_external() {
            let json = r#"
            {
                "id": "325708",
                "type": "image",
                "contentProvider": {
                    "type": "external",
                    "originalContentUrl": "https://path.to/original",
                    "previewImageUrl": "https://path.to/preview"
                }
            }
            "#;
            let res: Message = serde_json::from_str(json).expect("not formatted properly");

            assert_eq!(res, Message::Image {
                id: String::from("325708"),
                content_provider: ContentProvider::External {
                    original_content_url: String::from("https://path.to/original"),
                    preview_image_url: String::from("https://path.to/preview"),
                },
            });
        }

        #[test]
        fn test_deserialize_video_message_from_line() {
            let json = r#"
            {
                "id": "325708",
                "type": "video",
                "duration": 60000,
                "contentProvider": {
                    "type": "line"
                }
            }
            "#;
            let res: Message = serde_json::from_str(json).expect("not formatted properly");

            assert_eq!(res, Message::Video {
                id: String::from("325708"),
                duration: 60000,
                content_provider: ContentProvider::Line,
            });
        }

        #[test]
        fn test_deserialize_audio_message_from_line() {
            let json = r#"
            {
                "id": "325708",
                "type": "audio",
                "duration": 60000,
                "contentProvider": {
                    "type": "line"
                }
            }
            "#;
            let res: Message = serde_json::from_str(json).expect("not formatted properly");

            assert_eq!(res, Message::Audio {
                id: String::from("325708"),
                duration: 60000,
                content_provider: ContentProvider::Line,
            });
        }


        #[test]
        fn test_deserialize_file_message() {
            let json = r#"
            {
                "id": "325708",
                "type": "file",
                "fileName": "file.txt",
                "fileSize": 2138
            }
            "#;
            let res: Message = serde_json::from_str(json).expect("not formatted properly");

            assert_eq!(res, Message::File {
                id: String::from("325708"),
                file_name: String::from("file.txt"),
                file_size: 2138,
            });
        }

        #[test]
        fn test_deserialize_sticker_message() {
            let json = r#"
            {
                "id": "325708",
                "type": "sticker",
                "packageId": "1",
                "stickerId": "2"
            }
            "#;
            let res: Message = serde_json::from_str(json).expect("not formatted properly");

            assert_eq!(res, Message::Sticker {
                id: String::from("325708"),
                package_id: String::from("1"),
                sticker_id: String::from("2"),
            });
        }
    }
}