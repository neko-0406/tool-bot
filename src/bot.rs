use crate::interface::{GatewayResponse, Opcode};
use crate::opcode_event::opcode10_event;
use anyhow::Context;
use futures_util::stream::StreamExt;
use reqwest::{header::{self, HeaderMap, HeaderName, HeaderValue}, Client, Url};
use std::env;
use std::str::FromStr;
use tokio_tungstenite::{connect_async, tungstenite::Message};

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

    pub async fn login(&self) -> anyhow::Result<()> {
        // websocket用URLの取得
        let gateway_url = self.get_gateway_url().await?;

        // Gatewayに接続
        let (websocket_stream, _) = connect_async(&gateway_url.url)
            .await
            .with_context(|| "failed to connect websocket")?;
        println!("WebSocket is connected!!");

        let (mut write, mut read) = websocket_stream.split();

        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let data_str = str::from_utf8(&text.as_bytes())?;
                    let serialized_data = serde_json::from_str::<Opcode>(&data_str)?;
                    match serialized_data.op {
                        10 => opcode10_event(&serialized_data).await?,
                        _ => {}
                    }
                },
                Ok(Message::Close(_)) => {break;},
                Err(error) => {
                    println!("{:#?}", error.to_string());
                    break;
                },
                _ => { break; }
                
            }
            // 終了フラグがtrueなら終了
            if self.shutdown {
                break;
            }
        }
        Ok(())
    }

    async fn get_gateway_url(&self) -> anyhow::Result<GatewayResponse> {
        // gateway_url取得用のURLを作成
        let end_point = env::var("DISCORD_API_END_POINT")
            .with_context(|| "DISCORD_API_END_POINT is not found..".to_string())?;
        let gateway_url = Url::parse(&format!("{}{}", end_point, "/gateway/bot"))
            .with_context(|| "failed to create endpoint url..".to_string())?;

        // 認証用の値
        // user-agent用の値
        let repository_url = env::var("REPOSITORY_URL")
            .with_context(|| "repository url is not found..".to_string())?;
        let bot_version = env::var("BOT_VERSION")
            .with_context(|| "bot version is not found..".to_string())?;
        let user_agent = format!("DiscordBot ({}, {})", &repository_url, &bot_version);

        // headerの作成
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&self.auth_value).with_context(|| "failed to create header: auth".to_string())?
        );
        headers.insert(
            HeaderName::from_str("content-type").with_context(|| "failed to create header: content-type".to_string())?,
            HeaderValue::from_str("application/json").with_context(|| "failed to create header: content-type value".to_string())?
        );
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str(&user_agent).with_context(|| "failed to create header: user-agent")?
        );

        // http clientを作成して送信
        let client = Client::default();
        let request_builder = client.get(gateway_url);
        let response = request_builder
            .headers(headers)
            .send()
            .await?;

        let response_data = response.json::<GatewayResponse>().await?;

        Ok(response_data)
    }
}
