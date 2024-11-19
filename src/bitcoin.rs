use reqwest::Client;
use serde_json::Value;

pub async fn get_bitcoin_price(api_key: &str) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
    let client = Client::new();
    let response = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .query(&[("symbol", "BTC"), ("convert", "EUR")])
        .send()
        .await?
        .json::<Value>()
        .await?;

    response["data"]["BTC"]["quote"]["EUR"]["price"]
        .as_f64()
        .ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to parse Bitcoin price",
            )) as Box<dyn std::error::Error + Send + Sync>
        })
}
