# BTC-PriceBot

BTC-PriceBot est un bot Discord qui publie régulièrement le prix actuel du Bitcoin en euros, ainsi que la variation en pourcentage par rapport à la dernière mise à jour.

## Fonctionnalités

- Récupération du prix actuel du Bitcoin en utilisant l'API CoinMarketCap.
- Publication dans un canal Discord spécifique toutes les 30 minutes (ou un autre intervalle configurable).
- Indication de la variation en pourcentage par rapport à la mise à jour précédente.

## Prérequis

Avant de commencer, assurez-vous d'avoir les éléments suivants :

1. **Rust installé** : [Installer Rust](https://www.rust-lang.org/tools/install)
2. **Un compte Discord** :
   - Créez une application Discord dans le [portail développeur Discord](https://discord.com/developers/applications).
   - Configurez un bot et générez un **token**.
3. **Clé API CoinMarketCap** :
   - Inscrivez-vous sur [CoinMarketCap API](https://coinmarketcap.com/api/) et obtenez une clé API gratuite ou payante.
4. **Dépendances nécessaires** :
   - Installez les dépendances spécifiées dans `Cargo.toml`.


## Installation et Configuration

1. **Cloner le dépôt** :
    ```
   git clone https://github.com/votre-utilisateur/BTC-PriceBot.git
   cd BTC-PriceBot
    ```

2. **Créer un fichier `.env`** :
   - Créez un fichier `.env` dans le répertoire racine et ajoutez vos clés API et informations Discord :+
    ```
    DISCORD_TOKEN=your_discord_bot_token
    COINMARKETCAP_API_KEY=your_coinmarketcap_api_key
    ```
3. **Modifier le fichier de configuration** :
   - Dans le fichier `main.rs`, remplacez l'ID du canal par celui de votre serveur Discord :
    ```
    let channel_id = serenity::model::id::ChannelId(123456789012345678); // Remplacez par l'ID de votre canal
    ```  
4. Construire et exécuter le bot :
    ```
    cargo run
    ```
    - Le bot devrait se connecter à Discord et commencer à publier les prix du Bitcoin dans le canal spécifié.