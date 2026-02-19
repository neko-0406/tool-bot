use std::env;
use std::str::FromStr;
use reqwest::{Url, header::{self, HeaderMap, HeaderName, HeaderValue}, Client};
use crate::interface::GatewayResponse;

pub struct Bot {
    pub auth_value: String,
    pub websocket_url: String,
    pub shutdown: bool,
}

impl Bot {
    pub fn init(token: &str) -> Self {
        let auth_value = format!("Bot {}", token);
        Self {
            auth_value,
            websocket_url: String::new(),
            shutdown: false,
        }
    }

    pub fn login(&self) {}

    pub async fn get_gateway_url(&mut self) {
        // gateway_url取得用のURLを作成
        let end_point = env::var("DISCORD_API_END_POINT").expect("discord endpoint is not found..");
        let gateway_url = Url::parse(&format!("{}{}", end_point, "/gateway/bot"))
            .expect("failed to create endpoint url...");

        // 認証用の値
        let auth_value = self.auth_value.as_str();
        // user-agent用の値
        let repository_url = env::var("REPOSITORY_URL").expect("repository url is not found..");
        let bot_version = env::var("BOT_VERSION").expect("bot version is not found..");
        let user_agent = format!("DiscordBot ({}, {})", &repository_url, &bot_version);
        // headerの作成
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, HeaderValue::from_str(auth_value).expect("failed to create header"));
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
            self.websocket_url = json.url;
            // println!("{:#?}", json);
        }
    }
}
