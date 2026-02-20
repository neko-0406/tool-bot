use std::env;

use dotenv::dotenv;

use crate::bot::Bot;

mod bot;
mod interface;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env読み込み
    dotenv().ok();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Not Found Bot Token...");
    
    let bot = Bot::init(&token);
    bot.login().await?;
    Ok(())
}
