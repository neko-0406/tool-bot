use std::env;

use dotenv::dotenv;

use crate::bot::Bot;

mod bot;
mod interface;

#[tokio::main]
async fn main() {
    // .env読み込み
    dotenv().ok();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Not Found Bot Token...");
    
    let mut bot = Bot::init(&token);
    bot.get_gateway_url().await;
    // bot.login();
}
