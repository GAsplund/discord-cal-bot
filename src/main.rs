//use std::env;
mod config;
mod database;
mod discord;
mod google_calendar;
mod tasks;

use clokwerk::{AsyncScheduler, TimeUnits};
use std::sync::OnceLock;
use std::time::Duration;

use crate::discord::bot;
use crate::google_calendar::webhooks;

static WEBHOOK_URL: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    webhooks::run_webhook_server();

    let local_tunnel_url = webhooks::run_localtunnel();
    let _oauth_redirect = local_tunnel_url.clone() + "/oauth";
    let webhook_url = local_tunnel_url + "/webhook";
    let _ = WEBHOOK_URL.set(webhook_url);

    tokio::spawn(async {
        let mut scheduler = AsyncScheduler::new();

        scheduler.every(1.minutes()).run(tasks::update_events);
        scheduler.every(5.minutes()).run(tasks::update_watches);
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    tokio::join!(bot::start());
}
