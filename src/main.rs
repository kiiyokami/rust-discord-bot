use dotenv::dotenv;
use std::env;

mod bot;
mod services;
use bot::main::run_bot;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_token = env::var("DISCORD_TEST_TOKEN").expect("token not set");
    run_bot(&bot_token).await;
}