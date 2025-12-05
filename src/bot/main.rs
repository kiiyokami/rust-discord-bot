use serenity::{
    Client, all::{GatewayIntents}
};
use crate::bot::message_handler::Handler;


pub async fn run_bot(token: &String) {

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}