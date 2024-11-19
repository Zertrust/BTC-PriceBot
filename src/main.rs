mod bot;
mod bitcoin;
mod config;

use serenity::Client;
use serenity::model::gateway::GatewayIntents;
use bot::{Handler, start_notification_loop};
use config::{get_discord_token, get_coinmarketcap_api_key};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let discord_token = get_discord_token();
    let coinmarketcap_api_key = get_coinmarketcap_api_key();
    let channel_id = serenity::model::id::ChannelId(1308370726486544456); // Remplacez avec votre ID
    let notification_interval = 3600; // En secondes (60 minutes)

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;        
    let mut client = Client::builder(&discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let http = client.cache_and_http.http.clone();
    tokio::spawn(async move {
        start_notification_loop(&http, channel_id, &coinmarketcap_api_key, notification_interval).await;
    });

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
