//use std::env;
mod discord;
mod config;
mod google_calendar;

use discord::bot;
use crate::google_calendar::webhooks;
use crate::google_calendar::google_oauth;
use crate::google_calendar::calendar;

#[tokio::main]
async fn main() {
    let result = google_oauth::oauth().await.unwrap();
    println!("Code: {}, State: {}", result.code, result.state);
    calendar::do_call(result.code, result.state).await;
    tokio::join!(bot::start_bot(), webhooks::run_webhook_server(), webhooks::run_localtunnel() );
}
