use futures_util::stream::StreamExt;
use std::env;
use std::str::FromStr;
use reqwest::{Url, header::{self, HeaderMap, HeaderName, HeaderValue}, Client};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use crate::interface::{GatewayResponse, Opcode10};

pub struct Bot {
    pub auth_value: String,
    pub shutdown: bool,
}

impl Bot {
    pub fn init(token: &str) -> Self {
        let auth_value = format!("Bot {}", token);
        Self {
            auth_value,
            shutdown: false,
        }
    }

    pub async fn login(&self) {
        // websocket用URLの取得
        let gateway_url = match self.get_gateway_url().await {
            Some(gateway_response) => { gateway_response.url + "/?v=10&encoding=json" },
            None => { String::new() }
        };

        if gateway_url.is_empty() {
            println!("failed to get websocket url..");
            return;
        }

        // Gatewayに接続
        let (websocket_stream, _) = connect_async(&gateway_url)
            .await
            .expect("WebSocket接続に失敗しました");
        println!("WebSocket is connected!!");

        let (mut write, mut read) = websocket_stream.split();

        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Ok(text) = str::from_utf8(&text.as_bytes()) {
                        let json_data = serde_json::from_str::<Opcode10>(text);
                        match json_data {
                            Ok(data) => {println!("{:#?}", data)},
                            Err(error) => {
                                println!("{:#?}", error.to_string());
                                continue;
                            }
                        }
                    }
                },
                Ok(Message::Close(_)) => {break;},
                Err(error) => {
                    println!("{:#?}", error.to_string());
                    break;
                },
                _ => { break; }
                
            }
        }
        // let msg = read.next().await.unwrap().unwrap();
        // println!("{:#?}", msg);
    }

    async fn get_gateway_url(&self) -> Option<GatewayResponse> {
        // gateway_url取得用のURLを作成
        let end_point = env::var("DISCORD_API_END_POINT").expect("discord endpoint is not found..");
        let gateway_url = Url::parse(&format!("{}{}", end_point, "/gateway/bot"))
            .expect("failed to create endpoint url...");

        // 認証用の値
        // user-agent用の値
        let repository_url = env::var("REPOSITORY_URL").expect("repository url is not found..");
        let bot_version = env::var("BOT_VERSION").expect("bot version is not found..");
        let user_agent = format!("DiscordBot ({}, {})", &repository_url, &bot_version);
        // headerの作成
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&self.auth_value).expect("failed to create header"));
        headers.insert(HeaderName::from_str("content-type").unwrap(), HeaderValue::from_str("application/json").expect("failed to create header"));
        headers.insert(header::USER_AGENT, HeaderValue::from_str(&user_agent).expect("failed to create user agent"));

        // http clientを作成して送信
        let client = Client::default();
        let request_builder = client.get(gateway_url);
        let response = request_builder
            .headers(headers)
            .send()
            .await;

        // responseが成功したら展開
        if let Ok(response) = response {
            let json = response.json::<GatewayResponse>().await.expect("failed to serialize gateway response..");
            Some(json)
        } else {
            None
        }
    }
}
