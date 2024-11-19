use std::env;

pub fn get_discord_token() -> String {
    env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set in .env")
}

pub fn get_coinmarketcap_api_key() -> String {
    env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY not set in .env")
}
