use std::env;

use reqwest::{Url, header::{self, HeaderMap, HeaderName, HeaderValue}};

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

    pub fn login(&self) {}

    pub fn get_gateway_url(&self) {
        let end_point = env::var("DISCORD_API_END_POINT").expect("discord endpoint is not found..");
        let gateway_url = Url::parse(&format!("{}{}", end_point, "/gateway/bot"))
            .expect("faild to create endpoint url...");
        let auth_value = self.auth_value.as_str();
        
        let mut headers = HeaderMap::new();
    }
}
