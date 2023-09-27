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
    /*webhooks::run_webhook_server();

    let local_tunnel_url = webhooks::run_localtunnel();
    let oauth_redirect = local_tunnel_url + "/oauth";
    let webhook_url = local_tunnel_url + "/webhook";

    let (g_auth_url, client) = google_oauth::get_oauth_url(oauth_redirect).await;
    let result = google_oauth::oauth(&g_auth_url, &client).await.unwrap();
    println!("Code: {}, State: {}", result.code, result.state);
    calendar::do_call(result.code, &client).await;
    tokio::join!(bot::start_bot());*/

    /*match google_calendar::google_oauth::authenticate().await {
        Ok(token) => println!("The token is {:?}", token),
        Err(e) => println!("error: {:?}", e),
    }*/

    webhooks::run_webhook_server();

    let local_tunnel_url = webhooks::run_localtunnel();
    let _oauth_redirect = local_tunnel_url.clone() + "/oauth";
    let webhook_url = local_tunnel_url + "/webhook";

    let auth = google_oauth::authenticate()
        .await
        .expect("Failed to authenticate");

    let cal = calendar::get_cal(auth).await;
    calendar::watch(cal, &webhook_url, "primary").await;

    tokio::join!(bot::start_bot());
}
