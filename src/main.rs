//use std::env;
mod discord;
mod config;
mod google_calendar;
mod database;

use crate::google_calendar::webhooks;
use crate::google_calendar::google_oauth;
use crate::google_calendar::calendar;
use crate::discord::bot;
use crate::calendar::Calendar;

#[tokio::main]
async fn main() {
    webhooks::run_webhook_server();

    let local_tunnel_url = webhooks::run_localtunnel();
    let _oauth_redirect = local_tunnel_url.clone() + "/oauth";
    let webhook_url = local_tunnel_url + "/webhook";

    let auth = google_oauth::authenticate()
        .await
        .expect("Failed to authenticate");

    let cal = Calendar::from_auth(auth, "primary".to_string()).await;
    cal.watch(&webhook_url).await;

    tokio::join!(bot::start());
}
