use std::sync::{OnceLock, Arc};

use serenity::async_trait;
use serenity::http::Http;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use crate::config;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();

pub fn get_http() -> Option<Arc<Http>> {
    HTTP.get().cloned()
}

pub async fn start() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .group(&GENERAL_GROUP);

    let config = config::global();

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    println!("Starting Discord bot");
    let mut client = Client::builder(&config.bot_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = HTTP.set(client.cache_and_http.http.clone()) {
        println!("Error setting HTTP client: {:?}", why);
        return;
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
