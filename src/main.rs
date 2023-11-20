//use std::env;
mod config;
mod database;
mod discord;
mod google_calendar;

use crate::discord::bot;
use crate::google_calendar::calendar;
use crate::google_calendar::webhooks;

#[tokio::main]
async fn main() {
    webhooks::run_webhook_server();

    let local_tunnel_url = webhooks::run_localtunnel();
    let _oauth_redirect = local_tunnel_url.clone() + "/oauth";
    let webhook_url = local_tunnel_url + "/webhook";

    tokio::join!(bot::start());
}
