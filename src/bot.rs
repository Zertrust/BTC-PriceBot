use serenity::{
    async_trait,
    model::{gateway::Ready, id::ChannelId},
    prelude::*,
};
use tokio::time::{interval, Duration};
use crate::bitcoin::get_bitcoin_price;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn start_notification_loop(
    http: &serenity::http::Http,
    channel_id: ChannelId,
    api_key: &str,
    interval_secs: u64,
) {
    let mut interval = interval(Duration::from_secs(interval_secs));
    let mut previous_price: Option<f64> = None;

    loop {
        interval.tick().await;

        match get_bitcoin_price(api_key).await {
            Ok(current_price) => {
                let percentage_change = if let Some(prev) = previous_price {
                    Some(((current_price - prev) / prev) * 100.0)
                } else {
                    None
                };

                let content = if let Some(change) = percentage_change {
                    format!(
                        "Le prix moyen du Bitcoin est de {:.2}€. Variation par rapport à l'heure précédente : {:.2}%",
                        current_price, change
                    )
                } else {
                    format!(
                        "Le prix moyen du Bitcoin est de {:.2}€. (Pas de données de comparaison pour l'instant.)",
                        current_price
                    )
                };

                previous_price = Some(current_price);

                if let Err(why) = channel_id.say(http, content).await {
                    println!("Erreur lors de l'envoi du message : {:?}", why);
                }
            }
            Err(err) => {
                println!("Erreur lors de la récupération du prix Bitcoin : {:?}", err);
            }
        }
    }
}
